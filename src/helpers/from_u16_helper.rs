/// Attempts to convert a `u16` to an instance of the implemented type.
pub trait FromU16: Sized {
  fn from_u16(code: u16) -> Option<Self>;
}

pub fn try_from_u16<T: FromU16>(code: u16) -> Option<T> {
  T::from_u16(code)
}
