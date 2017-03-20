use std::env;
use std::fs::File;
use std::io::{Error, ErrorKind, BufReader};
use std::io::prelude::*;


const USAGE: &'static str ="
Eigene `cp` Implementierung in Rust. ACHTUNG unvollständig!\n
Usage:
\tmycp    <quelle>    <ziel>
";

fn check_args(args: env::Args) -> Result<env::Args, Error> {
    if args.len() == 3 {
        Ok(args)
    } else {
        Err(Error::new(ErrorKind::Other, "Argumente stimmen nicht. Bitte nur 2 Dateien angeben."))
    }
}

fn copy(args: env::Args) -> Result<(), Error> {
    let args: Vec<_> = args.collect();

    let mut buffer = Vec::new();
    try!(File::open(&args[1])).read_to_end(&mut buffer)?;
    try!(File::create(&args[2])).write(&mut buffer)?;

    Ok(())
}

fn main() {
    match check_args(env::args()) {
        Ok(args) => {
            match copy(args) {
                Ok(_) => { }
                Err(e) => { println!("{}", USAGE); println!("Error: {}", e); }
            }
        }
        Err(e) => { println!("{}", USAGE); println!("Error: {}", e); }
    }

}
