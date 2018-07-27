

fn main() {

//e6
//The sum of the squares of the first ten natural numbers is,
//1**2 + 2**2 + ... + 10**2 = 385
//The square of the sum of the first ten natural numbers is,
//(1 + 2 + ... + 10)**2 = 55**2 = 3025
//Hence the difference between the sum of the squares of the first ten natural numbers and the square of the sum is 3025 − 385 = 2640.
//Find the difference between the sum of the squares of the first one hundred natural numbers and the square of the sum.

	
	// //step by the iterator, squared and cubed respectively
	// fn ss_diff(x: i32) -> i32 {
	// 	let mut prev_val_sqs: i32 = 0;
	// 	let mut prev_val_ssq: i32 = 0;
	// 	let mut diff: i32 = 0;
	// 	let mut i: i32 = 0; 
	// 	while i < 101 {
	// 		let cur_val_sqs = prev_val_sqs + i.pow(3);
	// 		let cur_val_ssq = prev_val_ssq + i.pow(2);
	// 		diff = cur_val_sqs - cur_val_ssq;
	// 		prev_val_sqs = cur_val_sqs;
	// 		prev_val_ssq = cur_val_ssq;
	// 		i += 1;
	// 	}
	// 	diff
	// }

	// println!("{:?}", ss_diff(10) );

	//e7
	//By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can see that the 6th prime is 13.
	//What is the 10 001st prime number?

	// fn fac_len(mut x: i32) -> i32 {
	// 	let mut v: Vec<i32> = Vec::new();
	// 	let mut d = 2;
	// 	while x > 1 {
	// 		while x % d == 0 {
	// 			v.push(d);
	// 			x /= d
	// 		}
	// 		if v.len() > 1 { //break factor function as soon as we know it is not prime
	// 			break
	// 		}
	// 		d += 1
	// 	}
	// 	let size = v.len();
	// 	size as i32
	// }

	// //walk up and count primes until we reach 10001
	// fn get_fac(x: i32) -> i32 {
	// 	let mut count: i32 = 0; 
	// 	let mut iter: i32 = 1;
	// 	while count < x {
	// 		iter += 1;
	// 		if fac_len(iter) == 1 {
	// 			count += 1;
	// 		}
	// 	}
	// 	iter
	// }

	// println!("{:?}", get_fac(10001) );






	//e8
	//The four adjacent digits in the 1000-digit number that have the greatest product are 9 × 9 × 8 × 9 = 5832.

	// let data = 
	// "73167176531330624919225119674426574742355349194934
	// 96983520312774506326239578318016984801869478851843
	// 85861560789112949495459501737958331952853208805511
	// 12540698747158523863050715693290963295227443043557
	// 66896648950445244523161731856403098711121722383113
	// 62229893423380308135336276614282806444486645238749
	// 30358907296290491560440772390713810515859307960866
	// 70172427121883998797908792274921901699720888093776
	// 65727333001053367881220235421809751254540594752243
	// 52584907711670556013604839586446706324415722155397
	// 53697817977846174064955149290862569321978468622482
	// 83972241375657056057490261407972968652414535100474
	// 82166370484403199890008895243450658541227588666881
	// 16427171479924442928230863465674813919123162824586
	// 17866458359124566529476545682848912883142607690042
	// 24219022671055626321111109370544217506941658960408
	// 07198403850962455444362981230987879927244284909188
	// 84580156166097919133875499200524063689912560717606
	// 05886116467109405077541002256983155200055935729725
	// 71636269561882670428252483600823257530420752963450";

	// //Find the thirteen adjacent digits in the 1000-digit number that have the greatest product. What is the value of this product?

	// let char_vec: Vec<char> = data.chars().collect();
	// let mut run_vec: Vec<u32> = Vec::new(); //queue
	// let mut max_vec: Vec<u32> = Vec::new(); //highest slice
	// let mut max_prod: u32 = 0; 

	// for i in char_vec {
	// 	let int: Option<u32> = i.to_digit(10); //handle non-digit whitespace from input
	// 	if let Some(x) = int { //rust if let syntax is alternative to match
	// 	    run_vec.push( x );
	// 		if run_vec.len() > 13 {
	// 			run_vec.remove(0);
	// 			let mut zero: bool = false; //if 0 in slice product will be 0
	// 			for num in run_vec.iter() {
	// 				if *num == 0 as u32  {
	// 					zero = true;
	// 					break;
	// 				}
	// 			}
	// 			let prod = if zero { 0 }
	// 				else { run_vec.iter().fold(0, |acc, x| acc + x) };
	// 			if prod > max_prod { //found new highest slice
	// 				max_prod = prod;
	// 				max_vec.clear();
	// 				for digit in run_vec.iter() {
	// 					max_vec.push(*digit);
	// 				}
	// 			}
	// 		}
	//     }
	// }
	// println!("{:?}, {}", max_vec, max_prod );


	//e9
	//A Pythagorean triplet is a set of three natural numbers, a < b < c, for which,
	//a**2 + b**2 = c2
	//For example, 3**2 + 4**2 = 9 + 16 = 25 = 5**2.
	//There exists exactly one Pythagorean triplet for which a + b + c = 1000. Find the product abc.

	// fn gen_square_vec() -> Vec<i32> { //generate lookup array of squares
	// 	let mut a: i32 = 1;
	// 	let mut vec: Vec<i32> = Vec::new();
	// 	while a < 1000 {
	// 		vec.push(a.pow(2));
	// 		a += 1;
	// 	}
	// 	vec
	// }
	
	// fn iter_check() { //iter a and b in square lookup, print a+b+c=1000 to console
	// 	let squares_vec: Vec<i32> = gen_square_vec();
	// 	let num_squares = squares_vec.len() as i32;	
	// 	let mut a: i32 = 0;
	// 	let mut b: i32;
	// 	while a < num_squares { //outer loop a
	// 		b = a+1;
	// 		while b < num_squares { //inner loop b
	// 			if is_square(a,b,&squares_vec) { //lookup is c square 
	// 				let sum: i32 = sum(a, b, &squares_vec);
	// 				if sum == 1000 { // return
	// 					println!("a,b,c -->{}, {}, {}, {}",a+1, b+1, sum - (a+1) - (b+1), sum   );
	// 					return						
	// 				} 
	// 				else if sum > 1000 { //break to iter a
	// 					break
	// 				}
	// 			}
	// 			b += 1;
	// 		}
	// 		a += 1
	// 	}
	// 	println!("out of the loop");
	// }

	// fn is_square(a: i32, b:i32, vec:&Vec<i32>) -> bool { //is c square 
	// 	let aa = a as usize;
	// 	let bb = b as usize;
	// 	let ab:i32 = vec[aa] + vec[bb]; 
	// 	for i in vec.iter() {
	// 		if ab == *i {
	// 			return true
	// 		}
	// 	} 
	// 	false
	// }

	// fn sum(a: i32, b:i32, vec:&Vec<i32>) -> i32 { // sum of a, b, c
	// 	let aa = a as usize;
	// 	let bb = b as usize;
	// 	let ab:i32 = vec[aa] + vec[bb];
	// 	let mut c: i32 = 0;
	// 	for i in vec.iter() {
	// 		if ab == *i {
	// 			break
	// 		}
	// 		c += 1;
	// 	}
	// 	a + b + c + 3 //add 3 because a,b,c are for lookups in zero indexed array
	// }

	// iter_check( );


	//e10
	//The sum of the primes below 10 is 2 + 3 + 5 + 7 = 17.
	//Find the sum of all the primes below two million.

	

	fn is_prime(mut x: u64) -> bool {
		if x > 10 {
			if x % 2 == 0 {
				return false
			} else if x % 3 == 0 {
				return false
			}
		}

		let mut v: Vec<u64> = Vec::new();
		let mut d = 2;
		while x > 1 {
			while x % d == 0 {
				v.push(d);
				x /= d
			}
			if v.len() > 1 { //break factor function as soon as we know it is not prime
				return false
			}
			d += 1
		}
		v.len() == 1
	}

	//println!("{:?}", is_prime(2)  );

	fn collect_primes(x:u64) -> u64 {
		//let mut v: Vec<i32> =Vec::new();
		let mut sum: u64 = 0;
		for i in 1..x {
			if i % 50000 == 0 {
				println!("{:?}", i );
			}

			
			if is_prime(i) {
				//println!("{:?}",i );
				sum = kid_add(sum, i)
			}
		}
		sum
	}

	println!("{:?}", collect_primes(2000000) );

	use std::cmp;
	use std::char;


	fn kid_add(x:u64, y:u64) -> u64 {
		//println!("add {:?} + {:?}", x, y );

		let x_char_vec: Vec<char> = x.to_string().chars().rev().collect();
		let y_char_vec: Vec<char> = y.to_string().chars().rev().collect();
		//println!("{:?}, {:?}", x_char_vec, y_char_vec );
		let ln = cmp::max(   x_char_vec.len(), y_char_vec.len());
		let mut carry: u32 = 0;
		let mut result: Vec<char> = Vec::new();

		for ind in 0..ln {
			
			//println!("{:?} + {:?} + {:?}", char_to_dig( ind, &x_char_vec ) , char_to_dig( ind, &y_char_vec ) , carry  );

			let raw_digit = char_to_dig( ind, &x_char_vec ) + char_to_dig( ind, &y_char_vec ) + carry;
			carry = if raw_digit >= 10  { 1 } else {0};
			let digit = if raw_digit >= 10  { raw_digit - 10 } else { raw_digit};
			
			//println!("this is the digit {:?}", digit );
			//println!("this is the carry {:?}", carry );
			let c = char::from_digit( digit, 10);

			match c {
				Some(x) => result.push(x),
				None => println!("something went wrong 1"),
			}
		}
		if carry > 0 {
			let c = char::from_digit(carry, 10);
			//println!("this is the carry {:?}", c );
			match c {
				Some(x) => result.push(x),
				None => println!("something went wrong 2"),
			}
		}
		result.reverse();
		let result_str: String = result.iter().collect();
		let result_int: u64 =result_str.parse().unwrap();
		//println!("{:?}", result_int );
		result_int
	}


	fn char_to_dig( ind:usize, vec:&Vec<char> ) -> u32 {
		let dne = vec.get(ind);
		let x_val: u32 = match dne {
			Some(x) => {
				let digit: Option<u32> = x.to_digit(10);
				match digit {
					Some(x_dig) => x_dig,
					None => 0,
				} 
			},
			None => 0,
		};
		x_val
	}

	//println!("{:?}", kid_add(9999999999999999, 9) );

	//println!("{:?}", kid_add(921136, 99991) );

	 //921136 + 99991


}
