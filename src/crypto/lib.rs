#[derive(Debug, Display, PartialOrd, PartialEq, Eq)]
enum Mode {
    ECB,
    CBC,
    GCM
}

#[derive(Debug, Display, PartialOrd, PartialEq, Eq)]
enum KeyLength {
    bit128,
    bit160,
    bit256,
    bit512,
    bit1024,
    bit2048,
    custom(usize)
}

trait Encrypt {
    fn encrypt_inplace<'a>(data: &'a [u8], key: &[u8]) -> &'a [u8] {
    }

    // TODO: Should data go on stack or heap?
    fn encrypt(data: &[u8], key: &[u8]) -> [u8] {
    }

    fn encrypt_block(block: block) -> [u8] {

    }
}

// TODO: Make separate?
trait Decrypt {
    // add code here
}

// TODO: Trait for constant-time encryption?