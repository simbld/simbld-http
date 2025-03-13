/// Trait to convert a HttpCode into a simplified tuple with two fields.
pub trait IntoTwoFieldsTuple {
    fn into_two_fields_tuple(self) -> crate::helpers::two_fields_tuple_helper::TwoFieldsTuple;
}

/// Trait to convert a HttpCode into a simplified tuple with three fields.
pub trait IntoThreeFieldsTuple {
    fn into_three_fields_tuple(self)
        -> crate::helpers::three_fields_tuple_helper::ThreeFieldsTuple;
}
