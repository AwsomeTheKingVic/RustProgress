
// we use this std::io library to get input and send output from the user
use std::io;

//enum that does the less, greater, equl for you
use std::cmp::Ordering;

//using the random library that we added on to the project
use rand::Rng;

//main function where we run all the code in anything outside this scope wont run
fn main() {
	
	//just simple printstatement
    println!("Guess the number!");
	
	//chooses ad random number between the given length
	let secret_number = rand::thread_rng().gen_range(1..=100);
	
	//prints the secret number for we can see for testing
	//println!("The secret number is: {secret_number}");
	
	//will loop this section of the program
	loop {
	
		//ask the user for the number they guessed
		println!("Please input your guess.");
		
		//we are initualizing the guess variable witha string data type and letting it know it can be changed
		let mut guess = String::new();
		
		//so we are calling the function from io and saying we what to read the line coming in and make it take the value of guess by sending it the location that guess takes in memory allowing us 
		//to not need to recopy multiple times for the same affect causing less memory to be used and the expect says if anything wrong happends we will print the text inside of it
		//can be written as 
		//io::stdin().read_line($mut guess).expect("Failed to read line");
		//for readability use this
		io::stdin()
			.read_line(&mut guess)
			.expect("Failed to read line");
		
		//turned our guess number into a integer from the string it originally was
		let guess: u32 = match guess.trim().parse() {
			
			//will check if its a number will run next line otherwise will catch all errors and run the loop again
			Ok(num) => num,
			Err(_) => continue,
			
		};
			
		//print the variable
		println!("You guessed: {guess}");
		
		//will check the ref from secret_number and do the according checks work by using enums
		match guess.cmp(&secret_number){
			
			Ordering::Less => println!("Too small!"),
			Ordering::Greater => println!("Too big!"),
			Ordering::Equal => {
				
				//when the user matches it will break out of the loop
				println!("You win!");
				break;
			
			}
			
		}
		
	}
	
}
