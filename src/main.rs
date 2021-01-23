use std::io;

fn main() {
    println!("Please enter kelvin");

    let mut kelvin = String::new();

    io::stdin()
            .read_line(&mut kelvin)
            .expect("Failed to read line");

    let kelvin = kelvin.trim();
    let kelvin:f32 = kelvin.parse().unwrap();
    let celsius: f32 = kelvin - 273.0;

    let fahrenheit: f32 = celsius * 1.8 + 32.0; 
    
    let newton: f32 = celsius * 0.33;

    println!("The temperature converted from kelvin to celsius is {}. The temperature converted from celsius to fahrenheit is: {}", celsius, fahrenheit.floor());

    println!("The Newton temperature is: {}", newton.floor());
}
