const ALIVE_EYE: &str = "o";
const DEAD_EYE: &str = "x";

extern crate structopt;
extern crate colored;

use structopt::StructOpt;
use colored::*;
use std::io::Read;

const DEFAULT_CAT_TEMPLATE: &str = "\
\\
 \\
  /\\_/\\
 ( {eye} {eye} )
 =( I )=\
";

#[derive(StructOpt)]
struct Params {
    #[structopt(short = "m", long = "message")]
    message: Option<String>,
    #[structopt(short = "d", long = "dead")]
    dead: bool,
    #[structopt(short = "f", long = "file", parse(from_os_str))]
    cat_file: Option<std::path::PathBuf>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let params = Params::from_args();
    let mut message: String = String::new();
    match params.message {
        Some(msg) => message = msg,
        None => {
            std::io::stdin().read_to_string(&mut message)?;
        },
    } 

    println!("{}", message.blue());

    let cat_template: String;
    match params.cat_file {
        Some(path) => {
            cat_template = std::fs::read_to_string(path)?;
        },
        None => {
            cat_template = String::from(DEFAULT_CAT_TEMPLATE);
        }
    }

    let eyes = if params.dead { DEAD_EYE.red().bold() } else { ALIVE_EYE.green() };

    let cat_render = cat_template.replace("{eye}", &format!("{}", eyes) );
    println!("{}", cat_render);
    Ok(())
}
