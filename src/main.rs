//Temp Converter (F to C and C to F)
use std::io;

fn main() {
    println!("Temperature Converter!\n");

    loop {
        println!("Enter 1 for Celsius -> Fahrenheit\nEnter 2 for Fahrenheit to Celsius\n");

        let mut unit = String::new();
        let mut temp = String::new();

        io::stdin().read_line(&mut unit).expect("Error reading line");

        let unit: char = match unit.trim().parse() {
            Ok(1) => 'F',
            Ok(2) => 'C',
            _ => continue,
        };

        println!("Please input the temperature: ");

        io::stdin().read_line(&mut temp).expect("Error reading line");

        let temp: f64 = match temp.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match unit {
            'F' => {
                let converted_temp = (temp * 1.8) + 32.0;
                println!("{} degrees Celsius is {} degrees Fahrenheit", temp, converted_temp);
                break;
            },
            'C' => {
                let converted_temp = (temp - 32.0) * (5.0/9.0);
                println!("{} degrees Fahrenheit is {} degrees Celsius", temp, converted_temp);
                break;
            },
            _ => continue,
        };
    }
}
