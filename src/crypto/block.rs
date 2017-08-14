use std::slice;

#[derive(Clone, Debug, Eq, PartialEq)]
struct Block {
    pub data: slice::;
}

impl Block {
    // TODO: Exists a GetBytes trait? (pretty sure yes) (cf. chunks)
    fn new(data: [u8]) {

    }
}