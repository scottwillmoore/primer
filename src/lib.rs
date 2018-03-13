pub struct Prime {}

impl Iterator for Prime {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        Some(2)
    }
}

pub fn generate() -> Prime {
    Prime {}
}
