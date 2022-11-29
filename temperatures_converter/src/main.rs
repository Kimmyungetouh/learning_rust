use std::io;
fn fahrenheit_to_celsius(temperature:u16) -> f32{
    let result:f32 = ((temperature - 32) * (5 / 9)) as f32;
    return result;
}

fn celsius_to_fahrenheit(temperature:u16) -> f32{
    let result:f32 = (((temperature) * 9 / 5) + 32) as f32;
    return result;
}

fn main() {
    println!("*** Temperature converter ***");
    println!("Enter a temperature : ");
    let mut temperature:String = String::new();
    io::stdin().read_line(&mut temperature).expect("Failed to read temperature !");
    let temparature:u16 = match temperature.trim().parse::<u16>(){
      Ok(number) => number,
        Err(_) => panic!("Input a number as temperature please !")
    };

    println!("1. Celsius to Fahrenheit \n2. Fahrenheit to Celsius");
    let mut choice: String = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read choice !");
    let choice = match choice.trim().parse::<u8>() {
        Ok(number) => number,
        Err(_) => panic!("Choice must be 1 or 2 !")
    };

    let mut result= 0.00;
    if 1 == choice{
        result = celsius_to_fahrenheit(temparature)
    }else if 2 == choice{
        result = fahrenheit_to_celsius(temparature)
    }
    println!("Result: {}", result);
    }

