use std::io;

fn main() {
    println!("Hi there, Do you want to convert from °C to °F or °F to °C ? (1 or 2): ");

    let mut selected_value = String::new();

    io::stdin()
        .read_line(&mut selected_value)
        .expect("Failed to read line");

    let selected_value: i32 = match selected_value.trim().parse() {
        Ok(num) => num,
        Err(_) => 3,
    };

    if selected_value == 3 {
        println!("You entered a wrong character!")
    } else {
        println!("Enter value to convert : ");

        let mut entered_value = String::new();

        io::stdin()
            .read_line(&mut entered_value)
            .expect("Failed to read line");

        let entered_value: i32 = match entered_value.trim().parse() {
            Ok(num) => num,
            Err(_) => 3,
        };

        if selected_value == 1 {
            let result = celcius_to_farenheit(entered_value);

            println!("The result is {result}°F");
        }
        if selected_value == 2 {
            let result = farenheit_to_celcius(entered_value);

            println!("The result is {result}°C");
        }
    }
}

fn celcius_to_farenheit(celcius: i32) -> String {
    // General Formulae (°C to °F ) => (9°C/5 + 32) = °F

    let farenheit_value = (9.0 * celcius as f64 / 5.0) + 32.0;

    format!("{:.2}", farenheit_value)
}

fn farenheit_to_celcius(farenheit: i32) -> String {
    // General Formulae (°F to °C ) => (°F − 32) × 5/9 = °C

    let celcius_value = (farenheit as f64 - 32.0) * 5.0 / 9.0;

    format!("{:.2}", celcius_value)
}
