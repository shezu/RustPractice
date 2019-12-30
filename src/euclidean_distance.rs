
use std::io;

pub struct Coordinates {
    x1: isize,
    x2: isize,
    y1: isize,
    y2: isize,
}

impl Coordinates {
    pub fn get_distance() -> f64 {

        println!("Please enter the value of x1");

        let mut x1 = "".to_string();
        io::stdin().read_line(&mut x1).expect("failed to read");
        let x1: isize = x1.trim().parse().expect("Please enter a number");

        println!("Please enter the value of x2");

        let mut x2 = "".to_string();
        io::stdin().read_line(&mut x2).expect("failed to read");
        let x2: isize = x2.trim().parse().expect("Please enter a number");

        println!("Please enter the value of y1");

        let mut y1 = "".to_string();
        io::stdin().read_line(&mut y1).expect("failed to read");
        let y1: isize = y1.trim().parse().expect("Please enter a number");

        println!("Please enter the value of y2");

        let mut y2 = "".to_string();
        io::stdin().read_line(&mut y2).expect("failed to read");
        let y2: isize = y2.trim().parse().expect("Please enter a number");

        let coordinates = Coordinates { x1, x2, y1, y2 };
        let x = Coordinates::calculate_a_minus_b(coordinates.x2, coordinates.x1);
        let y = Coordinates::calculate_a_minus_b(coordinates.y2, coordinates.y1);

        let distance = ((x + y) as f64).sqrt();
        distance
    }

    fn calculate_a_minus_b(a: isize, b: isize) -> isize {
        let mut a_minus_b: isize = 0;
        let a_square = a * a;
        let b_square = b * b;
        let _2ab = 2 * a * b;

        println!(" a square : {}",a_square);
        println!(" b square : {}",b_square);
        println!(" _2ab : {}",_2ab);

        a_minus_b = (a_square - _2ab) + b_square;
        a_minus_b
    }
}
