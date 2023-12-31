use std::env;
use cli_utils::{run};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args : Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Too few arguments");
        return Ok(());
    }
    run(&args)
}


// echo(string),
// cat(string, string)
// ls(-)
// find(string)
// grep(string, string)
