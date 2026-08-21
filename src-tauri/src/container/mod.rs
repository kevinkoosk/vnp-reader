pub mod error;
pub mod integrity;
pub mod manifest;

use error::ContainerError;
use integrity::IntegrityDocument;
use manifest::{Manifest, ResourceCatalog, SceneCatalog};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::io::{Read, Seek};
use zip::ZipArchive;

pub const EXPECTED_MIMETYPE: &str = "application/vnd.visual-narrative-package+zip";

pub struct VnpPackage<R: Read + Seek> {
    archive: ZipArchive<R>,
    pub manifest: Manifest,
    pub scene_catalog: SceneCatalog,
    pub resource_catalog: ResourceCatalog,
    pub file_digests: HashMap<String, String>,
}

impl<R: Read + Seek> VnpPackage<R> {
    pub fn open(reader: R) -> Result<Self, ContainerError> {
        let mut archive = ZipArchive::new(reader)?;

        // 1. Validate paths across all entries
        Self::validate_entry_paths(&mut archive)?;

        // 2. Validate first entry mimetype
        Self::validate_mimetype(&mut archive)?;

        // 3. Read integrity catalog
        let integrity_doc =
            Self::read_json_entry::<IntegrityDocument>(&mut archive, "META-INF/integrity.json")?;
        if integrity_doc.algorithm.to_lowercase() != "sha-256" {
            return Err(ContainerError::MissingRequiredFile(
                "Unsupported integrity algorithm (only SHA-256 supported)".into(),
            ));
        }

        let mut expected_digests = HashMap::new();
        for entry in integrity_doc.entries {
            expected_digests.insert(entry.path, entry.digest.to_lowercase());
        }

        // 4. Verify and load manifest
        Self::verify_file_digest(&mut archive, "vnp/manifest.json", &expected_digests)?;
        let manifest = Self::read_json_entry::<Manifest>(&mut archive, "vnp/manifest.json")?;

        // 5. Verify and load catalogs
        Self::verify_file_digest(&mut archive, &manifest.catalogs.scenes, &expected_digests)?;
        let scene_catalog =
            Self::read_json_entry::<SceneCatalog>(&mut archive, &manifest.catalogs.scenes)?;

        Self::verify_file_digest(&mut archive, &manifest.catalogs.resources, &expected_digests)?;
        let resource_catalog =
            Self::read_json_entry::<ResourceCatalog>(&mut archive, &manifest.catalogs.resources)?;

        Ok(Self {
            archive,
            manifest,
            scene_catalog,
            resource_catalog,
            file_digests: expected_digests,
        })
    }

    pub fn read_verified_resource(&mut self, path: &str) -> Result<Vec<u8>, ContainerError> {
        Self::verify_file_digest(&mut self.archive, path, &self.file_digests)?;
        Self::read_raw_entry(&mut self.archive, path)
    }

    pub fn load_scene_json<T: for<'de> serde::Deserialize<'de>>(
        &mut self,
        path: &str,
    ) -> Result<T, ContainerError> {
        Self::verify_file_digest(&mut self.archive, path, &self.file_digests)?;
        Self::read_json_entry::<T>(&mut self.archive, path)
    }

    fn validate_entry_paths(archive: &mut ZipArchive<R>) -> Result<(), ContainerError> {
        for i in 0..archive.len() {
            let name = archive.by_index(i)?.name().to_string();
            if name.starts_with('/')
                || name.starts_with('\\')
                || name.contains("..")
                || name.contains("//")
            {
                return Err(ContainerError::UnsafePath(name));
            }
        }
        Ok(())
    }

    fn validate_mimetype(archive: &mut ZipArchive<R>) -> Result<(), ContainerError> {
        if archive.is_empty() {
            return Err(ContainerError::MissingRequiredFile("mimetype".into()));
        }

        {
            let entry = archive.by_index(0)?;
            if entry.name() != "mimetype" || entry.compression() != zip::CompressionMethod::Stored {
                return Err(ContainerError::InvalidMimetypeEntry);
            }
        }

        let mut content = String::new();
        archive.by_index(0)?.read_to_string(&mut content)?;

        if content != EXPECTED_MIMETYPE {
            return Err(ContainerError::MimetypeMismatch {
                expected: EXPECTED_MIMETYPE.into(),
                found: content,
            });
        }

        Ok(())
    }

    fn verify_file_digest(
        archive: &mut ZipArchive<R>,
        path: &str,
        expected_digests: &HashMap<String, String>,
    ) -> Result<(), ContainerError> {
        let expected = expected_digests
            .get(path)
            .ok_or_else(|| ContainerError::MissingRequiredFile(format!("Digest missing for {}", path)))?;

        let bytes = Self::read_raw_entry(archive, path)?;
        let mut hasher = Sha256::new();
        hasher.update(&bytes);
        let computed = hex::encode(hasher.finalize()).to_lowercase();

        if &computed != expected {
            return Err(ContainerError::DigestMismatch {
                path: path.into(),
                expected: expected.clone(),
                computed,
            });
        }

        Ok(())
    }

    fn read_raw_entry(
        archive: &mut ZipArchive<R>,
        path: &str,
    ) -> Result<Vec<u8>, ContainerError> {
        let mut file = archive
            .by_name(path)
            .map_err(|_| ContainerError::MissingRequiredFile(path.into()))?;
        let mut buffer = Vec::with_capacity(file.size() as usize);
        file.read_to_end(&mut buffer)?;
        Ok(buffer)
    }

    fn read_json_entry<T: for<'de> serde::Deserialize<'de>>(
        archive: &mut ZipArchive<R>,
        path: &str,
    ) -> Result<T, ContainerError> {
        let bytes = Self::read_raw_entry(archive, path)?;
        serde_json::from_slice(&bytes).map_err(|source| ContainerError::Json {
            path: path.into(),
            source,
        })
    }
}
