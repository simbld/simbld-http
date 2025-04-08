// Common trait to convert an enum to u16
pub trait ToU16 {
    fn get_code(&self) -> u16;
}
