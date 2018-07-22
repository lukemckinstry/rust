

fn main() {
    
	use std::io;
	use std::collections::HashMap;

	let mut results:HashMap<String, Vec<std::string::String>> = HashMap::new();

	loop {
		println!("results hashmap ----> {:?}", results );

		println!("Enter your command 'Add _NAME_ to _DEPT_'"); //prompt user input

	 	let mut guess = String::new();

	    io::stdin().read_line(&mut guess)
	        .expect("Failed to read line");

		guess.pop(); 	//remove trailing space from input
		let split = guess.split(" ");
		let vec: Vec<&str> = split.collect();
		
		assert_eq!(vec.len(), 4, "spec is {}, input is {}", 4, vec.len()); //check input has 4 words
		
		let input_add: &str = &vec[0];
		let input_to: &str = &vec[2];

		let spec_add = String::from("add");
		let spec_to = String::from("to"); 

		let _slice_add = &spec_add[..];
		let _slice_to = &spec_to[..];

		//check input conforms to add ... to ... spec
		match input_add.as_ref() {
		    "add" => println!("add statement ok"),
		    _ => { 
		    	println!("improper statement");
		    	break;
		    }
		}
		match input_to.as_ref() {
		    "to" => println!("to statement ok"),
		    _ => { 
		    	println!("improper statement");
		    	break;
		    }
		}
		
		//add value based on old value to hashmap
		let input_name: &str = &vec[1];
		let input_dept: &str = &vec[3];
		let stub = results.entry( input_dept.to_string() ).or_insert( vec![] );
		stub.push( input_name.to_string() ) ;
	}

}
