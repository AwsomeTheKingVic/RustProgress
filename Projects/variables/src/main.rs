//runs the program
fn main() {
	
	//Scalar Types
	/*
	//varibles and mutability
	//anitiulizing x and allowing it to be changed using mut
    let mut x = 5;
		
	//printing x for the first time
	println!("The value of x is: {x}");
	
	//using xs mut ability to change it
	x = 6;
	
	//printing the new x
	println!("The value of x is: {x}");
	
	//example of a const
	const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
	
	println!("The Time: {THREE_HOURS_IN_SECONDS}");
	
	//shadowing
	let y = 5;
	
	let y = y + 1;
	
	{
		
		let y = y * 2;
		
		println!("The value of y in the inner scope is: {y}");
		
	}
	
	println!("The value of y is {y}");
	*/
	
//==================================================================================================================	
	
	//data types
	/*
	//must add a datetype to clearify what you are using
	let guess: u32 = "42".parse().expect("not a number!");
	
	println!("parse guess: {guess}");
	
	//Floating-Point Types
	let x = 2.0;//f64
	
	let y: f32 = 3.0;//f32
	
	println!("fx: {x}");
	println!("fy: {y}");

	//Numeric Operations
	//addition
	let sum = 5 + 10;
	
	println!("sum: {sum}");
	
	//subtraction
	let difference = 95.5 - 4.3;
	
	println!("difference: {difference}");
	
	//multiplication
	let product = 4 * 30;
	
	println!("product: {product}");
	
	//devision
	let quotient = 56.7 / 32.2;
	let truncated = -5 / 3; //results in -1
	
	println!("quotient: {quotient}");
	println!("truncated: {truncated}");
	
	//remainder
	let remainder = 43 % 5;
	
	println!("remainder: {remainder}");
	
	//boolean
	let t = true;
	let f: bool = false;// with annotation
	
	print!("bt: {t}");
	print!("bf: {f}");
	
	//Character types
	let c = 'z';
	let z: char = 'x'; //with annotation
	let heart_eyed_cat = '😻';
	
	print!("cc: {c}");
	print!("cz: {z}");
	print!("cheart_eyed_cat: {heart_eyed_cat}");
	*/
	
//==================================================================================================================

	//Compound Types
	//Tuple type can not be used like array are set and used no moving around
	let _tup: (i32, f64, u8) = (500, 6.4, 1);
	
	//println!("Tuple: {tup}");
	
	//we can set numbers in to a tuple and then give them varibles later
	let tup = (500, 6.4, 1);
	
	let (_x, y, _z) = tup;
	
	println!("The Value of y from the tup is: {y}");
	
	//we can also use them like this
	let x: (i32, f64, u8) = (500, 6.4, 1);
	
	//can asses an element directly
	let five_hundred = x.0;
	
	println!("The Value x in place 0 is: {five_hundred}");
	
	let six_point_four = x.1;
	
	println!("The Value x in place 1 is: {six_point_four}");
	
	let one = x.2;
	
	println!("The Value x in place 2 is: {one}");
	
	//Array Types
	//arrays are fixed
	let _a = [1,2,3,4,5];

	//use arrays for constant know varibles if need be use vector for dynamic in and out
	let _months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];

	//you can assign the datetype and number of elements
	let _a: [i32; 5] = [1, 2, 3, 4, 5];	
	
	//if you need to have an array that fill its self with constant number you can do this number or other data and then specify length
	let _a = [3; 5];
	
	//you can access array varibles
	let a = [1, 2, 3, 4, 5];
	
    let first = a[0];
    let second = a[1];
	
	println!("first: {first}");
	println!("second: {second}");
	
	Desktop\Rust\RustProgress\Projects\variables
	
}
