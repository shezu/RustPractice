
use std::io;
pub fn repetitively(){
	
	//Read line

	println!("Enter the string you want to be repeated");

	let mut repeat = "".to_string();
	io::stdin().read_line(&mut repeat).expect("failed to read");
	repeat = repeat.trim().to_string();

	println!("Enter times u want it to be repeated");

	let mut n = "".to_string();
	io::stdin().read_line(&mut n).expect("failed to read");
	
	// convert string input to integer.
	let n: u32 = n.trim().parse().expect("Please type a number");
	
	for _ in 0..n {
		print!("{}",repeat);
	}
}