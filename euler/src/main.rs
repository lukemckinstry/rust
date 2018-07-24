fn main() {

	//2 fibonacci
	// fn fib(z: i32) -> i32 {
	// 	if z==0 { return 0 } 
	// 	else if z==1 { return 1 }
	// 	else { return fib(z-1) + fib(z-2) }
	// }

	// let mut counter = 0;
	// let mut this_fib = fib(0);
	// let mut even_sum = 0;
	// let mut done = false; // mut done: bool
	// while !done {
	// 	println!("fib of {} is {:?} and total is {}",counter, this_fib, even_sum );
	// 	if counter % 2 == 0 { even_sum += this_fib }
	// 	counter += 1;
	// 	this_fib = fib(counter);
	// 	if this_fib > 4000000 { done = true; }
	// }
	// println!("final sum of fibs {:?}", even_sum);

	//3 The prime factors of 13195 are 5, 7, 13 and 29.
	//What is the largest prime factor of the number 600851475143 ?

	// fn fac(mut x: i64) -> Vec<i64> {
	// 	let mut v: Vec<i64> = Vec::new();
	// 	let mut d = 2;
	// 	while x > 1 {
	// 		while x % d == 0 {
	// 			v.push(d);
	// 			x /= d
	// 		}
	// 		d += 1
	// 	}
	// 	v
	// }
	
	// println!("{:?}", fac(9009) );	
	//prime_fac(4.0)

	//4 A palindromic number reads the same both ways.
	//The largest palindrome made from the product of two 2-digit numbers is 9009 = 91 × 99.
	//Find the largest palindrome made from the product of two 3-digit numbers.

	//run backwards from 1 million
	//if number is a palindrome
	//get list of pairs of 3-digit factors for number

	// let s: u32 = 999999;
	// let char_vec: Vec<char> = s.to_string().chars().collect();
 	// let rev_char_vec: Vec<char> = s.to_string().chars().rev().collect();	
	// println!("is {:?} equal to {:?}???", char_vec, &rev_char_vec );
	// println!("{:?}", char_vec.eq( &rev_char_vec ) );

	// #[derive(Debug)]
	// struct Factor {
	//     f1: i32,
	//     f2: i32,
	//     prod: i32,
	// }

	// //get factors with i32 type
	// fn fac3digit(mut x: i32) -> Vec<Factor> {
	// 	let mut v: Vec<Factor> = Vec::new();
	// 	let mut d = 999;
	// 	while d > 99 {
	// 		while x % d != 0  {
	// 			d-=1
	// 		}
	// 		if x/d > 999 {
	// 			d-=1
	// 		} else {
	// 			let factor: Factor = Factor { f1: d, f2: x/d, prod:x };
	// 			v.push(factor);
	// 			d-=1;		
	// 		}
	// 	}
	// 	v
	// }

	// let mut s: i32 = 1000000;
	// while s > 1 {
	// 	s -= 1;
	// 	let char_vec: Vec<char> = s.to_string().chars().collect(); // turn i32 to char vector
 //    	let rev_char_vec: Vec<char> = s.to_string().chars().rev().collect(); //reverse copy
 //    	if char_vec.eq( &rev_char_vec ) { //check for palindrome
 //    		let pal: String = char_vec.iter().collect(); //char vec back to string 
 //    		let pal_num: i32 = pal.parse().unwrap(); //string to i32
 //    		let val_res: Vec<Factor> = fac3digit(pal_num); //get pair of 3-digit factors for number
 //    		if val_res.len() > 0 {
 //    			println!("{:?}", val_res );
 //    			break
 //    		}
 //    	}
	// }



	//5 
	//count by the largest prime

	let v10 = vec![10, 9, 8, 7, 6];
	let v20 = vec![20, 19,18,17,16, 15, 14,13,12,11];

	let mut chuck = 19;

	while true {
		let mut ok = true;
		for i in v20.iter() {
			if chuck % i != 0 {
				ok = false;
				break
			}
		}
		if ok {
			println!("{:?}", chuck ); 
			break
		}
		chuck += 19
	}



}
