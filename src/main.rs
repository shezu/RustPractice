extern crate chrono;

//29 Dec Assignments
mod count_chars;
mod print_stars_pattern;
mod add_up_numbers;
mod dates;

//17 Dec Assignments
mod euclidean_distance;
mod palindrome_checker;
mod repetitive_string;

// 5 Jan Assignments :: Generics & Traits
mod generics;
mod traits;

use traits::{Book,BookInformation};

fn main() {
    //29 Dec Assignments
    // Assignment 1 : 
    // count_chars::check_string_characters();

    // Assignment 2 : 
    // print_stars_pattern::print_stars();

    // Assignment 3 :
    // add_up_numbers::add_numbers();
    
    // Assignment 4 :
    // dates::get_distance_in_dates();


    //17 Dec Assignments
    // QUESTION : Check if entered string is palindrome;
	// palindrome_checker::palindrome();
	
	// QUESTION : Print entered string repetitively
	// repetitive_string::repetitively();

	// QUESTION : Euclidean Distance calculation
	// let distance = euclidean_distance::Coordinates::get_distance();
    // println!("distance is {}",distance);
    
    //12th JANUARY 2020 ASSIGNMENTS
    //closures()

    //5th JANUARY ASSIGNMENTS
    // TASK 1 
    // IMPLEMENT PYTHAGORUS THEOREM WITH GENERIC TRIANGLE
    // let triangle_int = generics::RightAngleTriangle{ base:7, perpendicular:13 };
    // println!("Hypotenuse is {}",triangle_int.calculate_hypotenus());

    // let triangle_float = generics::RightAngleTriangle{ base:7.0, perpendicular:13.0 };
    // println!("Hypotenuse is {}",triangle_float.calculate_hypotenus());

    //TASK 2
    //IMPLEMENT TRAITS
    let author = String::from("Shezu");
    let publisher = String::from("Shezu");

    let book_implementer : Book =  Book{author: author, publisher:publisher};
    traits::print_book_info(book_implementer);
}

fn closures(){
    //ASSIGNMENT 1
    // WRITE A CLOSURE THAT SUMS 2 NUMBERS
    let sum_closure = | i : i32 , j : i32 | -> i32 {
        i + j    
    };
    let sum = sum_closure(5,6);
    println!("sum is {}",sum);


    //ASSIGNMENT 2
    // WRITE A CLOSURE THAT CHECKS IF A NUMBER IS PRIME
    let prime_closure = |prime : i32| -> bool {

        if prime == 0 {
            print!("{} is a not prime",prime);
            return false
        }

        if prime == 1 {
            print!("{} is a prime",prime);
            return true
        }

        let mut is_prime = true;
        for i in 2..9{
            if i != prime && prime % i == 0 {
                is_prime = false;
                break
            }
        }
        if is_prime {
            println!("{} is a prime",prime);
        }else{
            println!("{} is a not prime",prime);
        }
        
        return is_prime
    };

    prime_closure(6);
}