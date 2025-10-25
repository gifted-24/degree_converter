use std::io;

fn input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut buffer: String = String::new();
    io::stdin().read_line(&mut buffer)
        .expect("Failed to read line");
    String::from(buffer.trim())
}

fn select() -> String {
    let options: [String; 3] = [String::from("A"), String::from("B"), String::from("C")];
    let selected: String = loop {
        let choice: String = input("convert degree from:\n   A. Celsius -> Fahrenheit\n   B. Fahrenheit -> Celsius\n   C. Exit\nChoose from options [A, B, C]: ");
        if choice == options[0] {
            break String::from("°C");
        } else if choice == options[1] {
            break String::from("°F");
        } else if choice == options[2] {
            break String::from("EXIT");
        } else {
            println!("\nInvalid Choice: [{}]\n", choice);
        }
    };
    selected
}

fn convert(from: f64, unit: String) {
    if unit == "°F" {
        let celsius_value: f64 = (from - 32.0)*(5.0/9.0);
        println!("----- {}{} -> {}{} -----\n", from, unit, celsius_value, "°C");
    } else {
        let fahrenheit_value: f64 = ((9.0 * from)/5.0) + 32.0;
        println!("----- {}{} -> {}{} -----\n", from, unit, fahrenheit_value, "°F");
    }
}

fn main() {
    loop {
        let unit: String = select();
        if unit == String::from("EXIT") {
            break;
        }
        let value: f64 = input("value: ").parse()
            .expect("Not a float [f64]");
        convert(value, unit);
    }
    println!("Thanks for using DEGREE CONVERTER!");
}
