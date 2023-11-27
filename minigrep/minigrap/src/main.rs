//allows us to collect arguments
use std::env;

//use to leave the program without casusing it to panic
use std::process;

//use thing in our project folder
use minigrap::Config;

fn main() {
	
	//collect console arguments in order to find location and file
	let args: Vec<String>  =  env::args().collect();
	
	//will call the contructer for the struct and if its not able to do so it will exit the program and let the user know
	let config= Config::new(&args).unwrap_or_else( |err: &str| {
		
		eprintln!("Problem parsing arguments: {}", err);
		process::exit(1);
		
	});
	
	//prints out the information to see
    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);
	
	//checks if their is an error returned
	if let Err(e) =  minigrap::run(config) {
		
		eprintln!("Application error: {}", e);
		process:: exit(1);
		
	}
	
}