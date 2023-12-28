//allows us to serialize and deserialize the data we are getting
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
//struct to hold the data
struct Classes {

	index: String,
	name: String,
	URL:  String,
	
}

//set up tokio run time allows us to use async and wait
#[tokio::main]

async fn main()  -> Result<(), reqwest::Error>{
    
	//makes a Client and stores data in todos
	let todos: Vec<Classes>  = reqwest::Client::new()
		//gets data for client from website
		.get("https://www.dnd5eapi.co/api/classes/")
		//send request
		.send()
		//waits for request
		.await?
		//turns jason to string
		//.text()
		//converts the data into jason
		.json()
		//waits for apporation to be finnished
		.await?;
		
	//print out todos
	println!("{:#?}", todos);
	
	Ok(())
	
}
