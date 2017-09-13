extern crate secstr;

use std::path::Path;
use secstr::SecStr;

#[derive(Clone, Debug, Eq, PartialEq)]
struct Config<'a> {
    password: SecStr,
    in_file: &'a str,
    out_file: &'a str,
    mode: Mode,
    keysize: KeySize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum Mode {
    ECB,
    CBC,
    GCM,
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum KeySize {
    AES160,
    AES256,
}
