#[cfg(test)]
mod tests {
    use super::*;
    use sha2::{Digest, Sha256};
    use std::io::{Cursor, Write};
    use zip::write::SimpleFileOptions;
    use zip::ZipWriter;

    fn sha256_hex(data: &[u8]) -> String {
        let mut hasher = Sha256::new();
        hasher.update(data);
        hex::encode(hasher.finalize())
    }

    #[test]
    fn test_valid_vnp_package_loading() {
        let mut buffer = Cursor::new(Vec::new());
        let mut writer = ZipWriter::new(&mut buffer);

        // 1. Write uncompressed mimetype[cite: 5]
        let stored_opt = SimpleFileOptions::default().compression_method(zip::CompressionMethod::Stored);
        writer.start_file("mimetype", stored_opt).unwrap();
        writer.write_all(EXPECTED_MIMETYPE.as_bytes()).unwrap();

        // 2. Prepare mock files[cite: 5]
        let manifest_json = r#"{
            "format": "vnp",
            "format_version": "1.0.0",
            "id": "com.test.sample",
            "version": "1.0.0",
            "title": "Test Visual Novel",
            "default_language": "en",
            "entry_scene": "start",
            "catalogs": {
                "scenes": "vnp/scenes.json",
                "resources": "vnp/resources.json",
                "state": "vnp/state.json"
            },
            "capabilities": {
                "required": ["vnp.core@1"]
            }
        }"#;

        let scenes_json = r#"{
            "scenes": {
                "start": { "href": "vnp/scenes/start.json", "mode": "adv" }
            }
        }"#;

        let resources_json = r#"{
            "resources": {}
        }"#;

        // 3. Compute digests for integrity.json[cite: 5]
        let manifest_digest = sha256_hex(manifest_json.as_bytes());
        let scenes_digest = sha256_hex(scenes_json.as_bytes());
        let resources_digest = sha256_hex(resources_json.as_bytes());

        let integrity_json = format!(
            r#"{{
                "algorithm": "sha-256",
                "entries": [
                    {{ "path": "vnp/manifest.json", "size": {}, "digest": "{}" }},
                    {{ "path": "vnp/scenes.json", "size": {}, "digest": "{}" }},
                    {{ "path": "vnp/resources.json", "size": {}, "digest": "{}" }}
                ]
            }}"#,
            manifest_json.len(),
            manifest_digest,
            scenes_json.len(),
            scenes_digest,
            resources_json.len(),
            resources_digest
        );

        let deflate_opt = SimpleFileOptions::default().compression_method(zip::CompressionMethod::Deflated);

        // 4. Write all entries to archive[cite: 5]
        writer.start_file("META-INF/integrity.json", deflate_opt).unwrap();
        writer.write_all(integrity_json.as_bytes()).unwrap();

        writer.start_file("vnp/manifest.json", deflate_opt).unwrap();
        writer.write_all(manifest_json.as_bytes()).unwrap();

        writer.start_file("vnp/scenes.json", deflate_opt).unwrap();
        writer.write_all(scenes_json.as_bytes()).unwrap();

        writer.start_file("vnp/resources.json", deflate_opt).unwrap();
        writer.write_all(resources_json.as_bytes()).unwrap();

        writer.finish().unwrap();

        // 5. Open and verify container[cite: 5]
        buffer.set_position(0);
        let package = VnpPackage::open(buffer).expect("Package failed validation");
        assert_eq!(package.manifest.title, "Test Visual Novel");
        assert_eq!(package.manifest.entry_scene, "start");
    }
}
