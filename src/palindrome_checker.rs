
use std::io;
pub fn palindrome() {
    
    println!("Enter text to be checked if Palindrome");

	//Read line
	let mut str = "".to_string();
	io::stdin().read_line(&mut str).expect("failed to read");
	str = str.trim().to_string();

	//Collect the reversed string into a variable , then make a vector to be able to get char at index.
	let reversed_str = str.trim().chars().rev().collect::<String>();
	let rev_chars: Vec<_> = reversed_str.chars().collect();

	let mut is_palindrome = true;
	for (i, c) in str.trim().chars().enumerate() {
		if c != rev_chars[i] {
			is_palindrome = false;
			break;
		}
	}

	if is_palindrome {
		println!("{} is a palindrome", str);
	}else{
		println!("{} is not a palindrome", str);
	}
}