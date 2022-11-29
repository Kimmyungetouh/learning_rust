use std::io;

fn fibonacci(number:u32)->u64{
    let mut result: u64 = 0;
  if number < 2{
      result = 1
  } else if number > 1{
      result = fibonacci(number-1) + fibonacci(number-2)
  }
    return result
}

fn fibonacci_suit(number:u32){
    // print!("{} ", fibonacci(0));
    for i in 0..number+1{
        print!("{} -> {},  ", i, fibonacci(i));
    }
}


fn main() {
    let mut nth_number: String = String::new();
    println!("Let's generate the nth Fibonacci number !");
    println!("Which nth Fibonacci number do you want : ");
    io::stdin().read_line(&mut nth_number).expect("Cannot read nth number you input !");
    let nth_number: u32 = match nth_number.trim().parse::<u32>(){
        Ok(num)=> num,
        Err(_) => todo!("Cannot parse nth_number !")
    };
    println!("The {} nth number of Fibonacci is {}", nth_number, fibonacci(nth_number));
    println!("Taste the suit...");
    fibonacci_suit(nth_number)
}

