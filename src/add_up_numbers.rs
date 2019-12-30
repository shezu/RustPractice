use std::io::stdin;
pub fn add_numbers(){

    println!("I will add up the number you give me");
    let mut keep_asking = true;
    let mut total_sum = 0;
    while keep_asking {
        println!("Number: ");
        let mut number = "".to_string();
        stdin().read_line(&mut number).expect("Invalid input");

        if let Ok(number) = number.trim().parse::<i32>(){
            if number == 0 {
                keep_asking = false;
            }else{
                total_sum += number;
                println!("The total so far is {}", total_sum);
            }
        }else{
            keep_asking = false;
        }
    }
    println!("The total is {}", total_sum);        
    
}