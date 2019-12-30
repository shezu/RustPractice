
use std::io::stdin;

pub fn check_string_characters(){

    println!("Enter the string you want to test !");
    let mut text = "".to_string();
    stdin().read_line(&mut text).expect("Error while reading user input");

    let (mut alphas, mut digits, mut spaces, mut specials) = (0,0,0,0);
    for c in text.trim().chars(){
        if c.is_alphabetic() {
            alphas += 1;
        }else if c.is_digit(10){
            digits += 1;
        }else if c == ' ' {
            spaces += 1;
        }else{
            specials += 1;
        }
    }

    println!("Numbers = {} , Alphabets = {} , Spaces = {} , Specials = {}", digits, alphas, spaces, specials);

}