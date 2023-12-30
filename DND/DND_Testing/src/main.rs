use DND_Testing;

#[tokio::main]
async fn main(){
	
	//checks for matches
	match DND_Testing::fetch_classes().await{
		
		//if mataches will print
		Ok(classes) => {
			
			for class in classes {
				
                println!(" Name: {}, URL: {}",  class.name, class.url);
				
            }
			
		}
		
		//if not will show error
		Err(err) => eprintln!("Error: {}", err),
		
	}

}

/*
//uncomment if not returning clases
fn main(){

	let result = tokio::runtime::Runtime::new().unwrap().block_on(DND_Testing::fetch_classes());
	
	//handles error
	if let Err(err) = result {
		
		eprintln!("Error: {}", err)
		
	}
	
}
*/
