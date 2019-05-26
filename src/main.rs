use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for items in list.iter() {
        if items > largest {
            largest = items;
        }
    }
    largest
}
struct Point<T> {
    x: T,
    y: T,
}

impl <T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value:i32) -> Guess{
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }
        Guess {
            value
        }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}

fn main() {
//    println!("Guess the number!");
//
//    let secret_number = rand::thread_rng().gen_range(1,101);
//
//    println!("The secret number is: {}", secret_number);
//    loop {
//        println!("Please input your guess");
//
//        let mut guess = String::new();
//
//        io::stdin().read_line(&mut guess)
//            .expect("Failed to read line");
//
//        let guess: Guess = match guess.trim().parse() {
//            Ok(num) => Guess::new(num),
//            Err(e) => {
//                println!("{}", e);
//                continue;
//            },
//        };
//
//        println!("You guessed: {}", guess.value);
//
//        match guess.value.cmp(&secret_number) {
//            Ordering::Less => println!("Too small"),
//            Ordering::Equal => {
//                println!("You win");
//                break;
//            },
//            Ordering::Greater => println!("Too big"),
//        }
//    }
    let p = Point { x: 5, y: 10};
    println!("p.x = {}", p.x());
}
