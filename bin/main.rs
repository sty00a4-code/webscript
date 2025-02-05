extern crate lalrpop_util;
extern crate wasmtime;
extern crate webscript_lang;

use lalrpop_util::lalrpop_mod;
use std::{env, fs, process};

lalrpop_mod!(pub webscript);

fn main() {
    let mut args = env::args().skip(1);
    if let Some(path) = args.next() {
        let text = fs::read_to_string(&path)
            .map_err(|err| {
                eprintln!("ERROR {path}: {err}");
                process::exit(1);
            })
            .unwrap();
        let module = webscript::ModuleParser::new()
            .parse(&text)
            .map_err(|err| {
                eprintln!("ERROR {path}: {err}");
                process::exit(1);
            })
            .unwrap();
        dbg!(module);
    } else {
        println!("{USAGE}");
    }
}
const USAGE: &str = r#"USAGE:
    ws <file> -- runs file
"#;
