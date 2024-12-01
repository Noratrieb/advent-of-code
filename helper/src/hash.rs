use std::hash::Hasher;

#[derive(Default)]
pub struct NoHasher {
    value: u64,
}

impl Hasher for NoHasher {
    fn finish(&self) -> u64 {
        self.value
    }

    fn write_u32(&mut self, i: u32) {
        self.value = i as u64;
    }

    fn write_u64(&mut self, i: u64) {
        self.value = i;
    }

    fn write(&mut self, _: &[u8]) {
        unimplemented!()
    }
}
