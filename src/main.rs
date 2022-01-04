/* minigrep: leave argument collecting and error handling
in src/main.rs */
use std::env; 
use std::process; 

use minigrep::Config; 

// CASE_INSENSITIVE=1 cargo run to poem.txt
fn main() {
    let args: Vec<String> = env::args().collect(); 

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err); 
        process::exit(1);
    }); 

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e); 
        process::exit(1); 
    }
}
