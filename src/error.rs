use failure_derive::Fail;

#[derive(Debug, Fail)]
pub enum HederaError {
    #[fail(display= "Missing field: {}", field)]
    MissingField{field: &'static str},

    #[fail(display= "expected string of the format: {}", format)]
    ImproperFormat{format: &'static str},

    #[fail(display= "expected exactly 3 numbers separated by ':'")]
    InvalidID,
}
