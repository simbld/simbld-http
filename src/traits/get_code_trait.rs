// Common trait to convert an enum to u16
pub trait GetCode {
    fn get_code(&self) -> u16;
}
