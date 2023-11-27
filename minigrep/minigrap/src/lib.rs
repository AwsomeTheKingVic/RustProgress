//this file just acts like a personal library for our function
//in order to use our custome library we must let them be public

//to read files
use std::fs;

//to use errors
use std::error::Error;

//allow us to use inviroment variables
use std::env;

//handles the printing and reading of files
pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
	
	//read file
	let contents: String= fs::read_to_string(config.filename)?;
	
	//check what the boolean is for the case sensitivety
	let results= if config.case_sensitive{
		
		search_case_sensitive(&config.query, &contents)
		
	}else{
		
		search_case_insensitive(&config.query,&contents)
		
	};

	//after doing a search will then run each line
	for line in results{
		
		println!("{}", line);
		
	}
	
	//if no error return empty
	Ok(())
	
}

//allow  us  to  hold  the  needed  informati on  in  one  area
pub struct Config {
	
	pub query:String,
	pub filename:String,
	pub case_sensitive: bool
	
}

//Function that works directly with the struct
impl Config {
	
	//will handle theincoming argumentsfrom the console
	//works as a  contruster
	pub fn new(args: &[String]) ->  Result<Config , &str>{
		
		//checks to see if we have less then 3 arguments
		if args.len() < 3 {
			
			return Err("not enough arguments");
			
		}
		
		//store the inportant console arguments as in what and where
		//weare cloning  for now as to not  deal  with  life times  at the moment
		let query: String = args[1].clone();
		let filename: String = args[2].clone();
		
		//checks for a key otherwise error
		//can set on console by typing set CASE_INSENSITIVE=true can be unset by set CASE_INSENSITIVE=
		let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
		
		//sets the values of the struct with the user strings
		Ok (Config{query, filename, case_sensitive})
		
	}
	
}

//search for all the query string in content is case sensitive
//assing life time 'a
pub fn search_case_sensitive<'a>(query: &str, contents:&'a str) -> Vec<&'a str>{
	
	//makes new vectort to store content
	let mut results = Vec::new();
	
	//for every line in content
	for line in contents.lines() {
			
		//if it is the same as the query push to are new vector
		if line.contains(query) {
			
			results.push(line);
			
		}
		
	}
	
	//return new vector
	results

}

//does the same thing the previous function will do but is not case sensitive
pub fn search_case_insensitive<'a>(query: &str,contents: &'a str,) -> Vec<&'a str>{
	
	//gets the query turns it to lower case
	let query = query.to_lowercase();
	
	//makes new vectort to store content
	let mut results = Vec::new();
	
	//for every line in content
	for line in contents.lines() {
			
		//first will take the line make it lower case then if it is the same as the query push to are new vector
		if line.to_lowercase().contains(&query) {
			
			results.push(line);
			
		}
		
	}
	
	//return new vector
	results

	
}

//to try test driven development
//configers it to test
#[cfg(test)]

//defining a module named test and importanting everything from parent 
mod tests{
	
	use super::*;
	
	//add test function
	#[test]
	fn case_sensitive(){
		
		let query: &str = "duct";
		
		//it is picky write like this to save headache
		let contents: &str = r"\		
Rust: 
safe, fast, productive.
pick three.
Duct tape.";
		
		assert_eq!(vec!["safe, fast, productive."], search_case_sensitive(query, contents));
		
	}
	#[test]
	fn case_insensitive(){
		
		let query: &str = "rUsT";
		
		let contents: &str = r"\		
Rust:
safe, fast, productive.
pick three.
Trust me.";
		
		assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(query, contents));
		
	}
	
}