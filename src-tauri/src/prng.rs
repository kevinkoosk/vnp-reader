use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VnpPrng1 {
    pub state: u32,
}

impl VnpPrng1 {
    pub fn new(initial_seed: u32) -> Result<Self, &'static str> {
        if initial_seed == 0 {
            return Err("VNP-PRNG1 seed cannot be zero.");
        }
        Ok(Self { state: initial_seed })
    }

    /// Step the 32-bit xorshift state once
    pub fn next_u32(&mut self) -> u32 {
        let mut x = self.state;
        x ^= x << 13;
        x ^= x >> 17;
        x ^= x << 5;
        self.state = x;
        self.state
    }

    /// Unbiased draw across a total positive weight
    pub fn draw_weighted(&mut self, total_weight: u32) -> u32 {
        assert!(total_weight > 0, "Total weight must be positive.");
        let limit = (u32::MAX / total_weight) * total_weight;
        loop {
            let output = self.next_u32();
            if output < limit {
                return output % total_weight;
            }
        }
    }
}
