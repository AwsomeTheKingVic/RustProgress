//Desktop\Rust\RustProgress\content_calculator\food_content_calculator
use std::io;

fn main() {
	
	/*
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
    
	println!("Press enter when done");
	
	loop{
		
		//user inputs ingredient amount
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
		
		//user inputs error
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
	
	//iterates through the vector printing out the index + 1, number, and error 
	for (index, number) in ingredient_content.iter().enumerate(){
		
		println!("ingredient: {} = content amount of: {} mg +/- error of: {}", index + 1, number, ingredient_content_error[index]);
		
	}
	*/
	
//===============================================================================================================================================
	
	let mut food_slice: Vec<Vec<f64>> = Vec::new();
	let mut food_proportion: Vec<f64> = Vec::new();
	
	println!("Press enter when done");
	println!("Enter Slice information ");
	
	loop{
		
		//holds the string from the user
		let mut input_proportion = String::new();
		
		//get user input_ingredient and check for failure
		io::stdin().read_line(&mut input_proportion).expect("Failed to read line");
		
		match input_proportion.trim().parse() {
				
			Ok(num) => food_proportion.push(num),
			Err(_) => {
				
				food_slice.push(food_proportion.clone());
				
				//food_proportion.clear();
				
				//add another slice info
				println!("Would you Like to add another(add) slice otherwise press enter");
		
				let mut exit = String::new();
				
				io::stdin().read_line(&mut exit).expect("Failed to read line");
				
				//continue the loop or break out
				match exit.trim() == "add" {
					
					true => { 
					
						println!("Enter Slice information ");
					
						continue;
						
					}
					
					_ => break,
					
				}
					
			}
			
		};
	
	}
	
	for i in food_slice.iter() {
		
		println!("Slice {}", i);
		
	}
	
	//print the vectors
	for i in 0..food_slice.len() {
		
		println!("Slice {}", i + 1);
		
		for o in 0..food_proportion.len() {
			
			println!("  {}", food_slice[i][o]);
			
		}
		
		println!("");
		
	}
	
}
