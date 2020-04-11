use std::io;

fn main() {
    println!("Choose conversion:");
    println!("1) From Celsius to Fahrenheit");
    println!("2) From Fahreinheit to Celsius");
    let mut option = String::new();
    io::stdin()
        .read_line(&mut option)
        .expect("Failed to read line");
    let option = option.trim();
    if option == "1" {
        println!("Type a temperature in Celsius");
    } else if option == "2" {
        println!("Type a temperature in Fahrenheit");
    }

    let mut temp = String::new();
    io::stdin()
        .read_line(&mut temp)
        .expect("Failed to read line");

    let temp: f64 = match temp.trim().parse() {
        Ok(num) => num,
        Err(_) => return,
    };
    let mut converted_temp: f64 = 0.0;

    if option == "1" {
        converted_temp = (temp * 9.0 / 5.0) + 32.0;
    } else if option == "2" {
        converted_temp = (temp - 32.0) * 5.0 / 9.0;
    }

    println!("The temperature in Fahreinheit is {}", converted_temp);
}
