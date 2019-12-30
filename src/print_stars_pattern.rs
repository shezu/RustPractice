pub fn print_stars(){
    let number_of_lines : i32 = 12;
    let middle_val : i32 = number_of_lines / 2;
    let mut stars_printed : usize = 0;

    for i in 1..=number_of_lines {
        
        if i <=  middle_val {
            stars_printed += 1;
            let str_to_print = "*".repeat(stars_printed);
            println!("{}", str_to_print);
        }else{
            stars_printed -= 1;
            if stars_printed == 0 {break;}
            let str_to_print = "*".repeat(stars_printed);
            println!("{}", str_to_print);
        }
    }

}