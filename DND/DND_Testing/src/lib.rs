//we will use reqwest for our http well requests
use reqwest;

//allows us to use json information
use serde::{Deserialize, Serialize};
use serde_json::Value;

//allows use to use debug functions
#[derive(Debug)]
//is the sturct wiht the information we wish to use later
pub struct Classes {
	
	pub index: String,
	pub name: String,
	pub url: String,
	
}

//implementing functions to our struct
impl Classes {
	
	//this functions get the data from the new objects and creates a class object that we can use
    fn from_json_object(value: &Value) -> Option<Classes> {
		
		//checks if the json had values to make a class object  otherwise it will do nothing
        if let (Some(index), Some(name), Some(url)) = (
		
            value["index"].as_str(),
            value["name"].as_str(),
            value["url"].as_str(),
			
        ) {
			
            Some(Classes {
				
                index: index.to_string(),
                name: name.to_string(),
                url: url.to_string(),
				
            })
			
        } else {
			
            None
			
        }
		
    }
	
}

//used to get the classes
pub async fn fetch_classes() -> Result<Vec<Classes>, Box<dyn std::error::Error>> {
	
	//links
    //let url = "https://www.dnd5eapi.co/api/ability-scores";
    //let url = "https://www.dnd5eapi.co/api/alignments";
    //let url = "https://www.dnd5eapi.co/api/backgrounds";
    let url = "https://www.dnd5eapi.co/api/classes";
    //let url = "https://www.dnd5eapi.co/api/conditions";
    //let url = "https://www.dnd5eapi.co/api/damage-types";
    // let url = "https://www.dnd5eapi.co/api/equipment";
    //let url = "https://www.dnd5eapi.co/api/equipment-categories";
    //let url = "https://www.dnd5eapi.co/api/feats";
    //let url = "https://www.dnd5eapi.co/api/features";
    //let url = "https://www.dnd5eapi.co/api/languages";
    //let url = "https://www.dnd5eapi.co/api/magic-items";
    //let url = "https://www.dnd5eapi.co/api/magic-schools";
    //let url = "https://www.dnd5eapi.co/api/monsters";
    //let url = "https://www.dnd5eapi.co/api/proficiencies";
    //let url = "https://www.dnd5eapi.co/api/races";
    //let url = "https://www.dnd5eapi.co/api/rule-sections";
    //let url = "https://www.dnd5eapi.co/api/rules";
    //let url = "https://www.dnd5eapi.co/api/skills";
    //let url = "https://www.dnd5eapi.co/api/spells";
    //let url = "https://www.dnd5eapi.co/api/subclasses";
    //let url = "https://www.dnd5eapi.co/api/subraces";
    //let url = "https://www.dnd5eapi.co/api/traits";
    //let url = "https://www.dnd5eapi.co/api/weapon-properties";
	
	//takes url link sends request waits converts json into usable values and waits again waits are for cathing errors
    let response = reqwest::get(url).await?.json::<Value>().await?;

	//then checks if results is in the json results is one of the values
    if let Some(results) = response.get("results") {
		
		//checks if the results value is an array in order to gather information to go into a class
        if let Some(results_array) = results.as_array() {

			//converts the array into class values that then we can use
            let classes: Vec<Classes> = results_array
                .iter()
                .filter_map(Classes::from_json_object)
                .collect();
			
			
			//handles the returns of the errors and returns classes
			Ok(classes)
			
			/*
			//if not returning classes uncomment
			//prints all the class information on to the console
            for class in classes {
				
                println!("Index: {}, Name: {}, URL: {}", class.index, class.name, class.url);
				
            }
			*/
		
		//if not an array gives out error  message
        } else {
			
            eprintln!("Unexpected format for 'results' field in JSON response");
			
			//return an error
			Err("Unexpected format".into())
			
        }
		
	//if no result value in json  gives out error  message
    } else {
		
        eprintln!("No 'results' field in JSON response");
		
		//return an error
		Err("No 'results' field".into())
		
    }
	
	/*
	//if not returning classes uncomment
	//handles the returns of the errors and returns classes
    Ok(())
	*/
		
}

/*
//this is for base use of the json without needing to go father in
#[derive(Debug, Serialize, Deserialize)]
//struct to hold the data
struct Classlist {

	count: u8,
	results: Vec<Classes>,
	
	
}

#[derive(Debug, Serialize, Deserialize)]
struct Classes {
	
	index: String,
	name: String,
	url: String,
	
}

#[tokio::main]
async fn main()  -> Result<(), Box<dyn std::error::Error>>{
    
	let url = "https://www.dnd5eapi.co/api/classes/";
	
	let resq =  reqwest::get (url)
		.await?
		.text()
		.await?;
		
	let person: Classlist = serde_json::from_str(&resq)?;
	
	println!("{:?}", person);
	
	Ok(())
	
}
*/