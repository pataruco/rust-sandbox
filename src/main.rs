fn main() {
    for i in 1..101 {
        match (i % 3, i % 5) {
            (0, 0) => println!("{} FizzBuzz", i),
            (0, _) => println!("{} Fizz", i),
            (_, 0) => println!("{} Buzz", i),
            (_, _) => println!("{}", i),
        }
    }
}
