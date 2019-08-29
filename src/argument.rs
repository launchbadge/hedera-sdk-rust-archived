
#[derive(Clone)]
pub(crate) struct Argument {
    pub(crate) value: Vec<u8>,
    pub(crate) dynamic: bool
}

impl Argument {
    pub(crate) fn new(val: Vec<u8>, dynam: bool) -> Self {
        if dynam == false && val.len() != 32 {
            panic!("ILLEGAL ARGUMENT ERROR: value argument that was not 32 bytes; value was \
            {:#?} bytes", val.len());
        }
        Self {
            value: val,
            dynamic: dynam
        }
    }
}
