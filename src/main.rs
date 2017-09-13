extern crate clap;

use clap::{App, Arg, ArgGroup};

fn main() {
    let encrypt = Arg::with_name("encrypt")
        .short("e")
        .long("encrypt")
        .group("operation");

    let decrypt = Arg::with_name("decrypt")
        .short("d")
        .long("decrypt")
        .group("operation");

    // TODO: Add verify operation
    let operation = ArgGroup::with_name("operation")
        .required(true)
        .args(&["encrypt", "decrypt"]);

    let mode = Arg::with_name("mode")
        .short("m")
        .long("mode")
        .takes_value(true)
        .value_name("mode")
        .possible_values(&["cbc", "ecb", "gcm"])
        .default_value("gcm")
        .help("Specifies the mode of encryption");

    // TODO: Finish the rest of the arguments
    let key = Arg::with_name("key")
        .short("k")
        .long("key")
        .takes_value(true)
        .value_name("FILE")
        .help("Provides a config file to myprog");

    let input = Arg::with_name("parallel")
        .short("p")
        .long("parallel")
        .help("Provides a config file to myprog");

    let output = Arg::with_name("output")
        .short("o")
        .long("output")
        .takes_value(true)
        .value_name("PATH")
        .help("Path for output file");

    let app = App::new("Advanced Encryption Standard")
        .version("0.1")
        .author("Jesse Wright")
        .about("A lightweight AES utility that to encrypt arbitrary data")
        .args(&[encrypt, decrypt, mode, key, input, output]);

    let options = &["operation", "mode", "key", "input", "output"];
    let matches = app.get_matches_from_safe(options);

    let matches = match matches {
        Ok(arg_matches) => arg_matches,
        Err(err) => {
            println!("Bad options: {:?}", err);
            std::process::exit(1);
            unreachable!();
        }
    };
}
