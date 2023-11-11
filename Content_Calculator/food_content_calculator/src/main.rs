//Desktop\Rust\RustProgress\content_calculator\food_content_calculator
use std::io;

fn main() {
	
	//holds the number of Ingredients and there content number
	let mut ingredient_content: Vec<f64> = Vec::new();
	let mut ingredient_content_error: Vec<f64> = Vec::new();
	
	
	let mut test2: Vec<Vec<i8>> = Vec::new();
	
	let mut test3: Vec<i8> = vec![1,2,3,4];
	
	for i in 0..3{
		
		test2.push(test3.clone());		
		
	}
	
	for i in 0..test2.len() {
		
		for o in 0..test3.len() {
			
			print!("{}", test2[i][o]);
			
		}
		
		println!("");
		
	}
    
	/*
	println!("Press enter when done");
	
	loop{
		
		println!("Enter ingredients content ammount ");
		
		//holds the string from the user
		let mut input_ingredient = String::new();
		
		//get user input_ingredient and check for failure
		io::stdin().read_line(&mut input_ingredient).expect("Failed to read line");
		
		match input_ingredient.trim().parse() {
				
			Ok(num) => ingredient_content.push(num),
			Err(_) => {
				
				break;
			
			}
			
		};
		
		println!("Error range here: ");
		
		//holds the string from the user
		let mut input_error = String::new();
		
		//get user input_error and check for failure
		io::stdin().read_line(&mut input_error).expect("Failed to read line");
		
		match input_error.trim().parse() {
				
			Ok(num) => ingredient_content_error.push(num),
			Err(_) => {
				
				break;
			
			}
			
		};
	
	}

	for (index, number) in ingredient_content.iter().enumerate(){
		
		println!("ingredient: {} = content amount of: {} mg +/- error of: {}", index + 1, number, ingredient_content_error[index]);
		
	}
	
	let mut ingredient_proportion: Vec<f64> = Vec::new();
	
	loop{
		
		println!("Enter ingredients content ammount ");
		
		//holds the string from the user
		let mut input_proportion = String::new();
		
		//get user input_ingredient and check for failure
		io::stdin().read_line(&mut input_proportion).expect("Failed to read line");
		
		match input_proportion.trim().parse() {
				
			Ok(num) => ingredient_proportion.push(num),
			Err(_) => {
				
				break;
			
			}
			
		};
	
	}
	
	*/
	
}
