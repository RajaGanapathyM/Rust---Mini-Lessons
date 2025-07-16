use std::io::{self,Write};

fn safe_division(dividend: i32, divisor: i32) -> Result<i32, String> {
    if divisor == 0 {
        Err("Division by zero is not allowed".to_string())
    } else {
        Ok(dividend / divisor)
    }
}

fn evaluate_expression(expr: &str) -> Result<i32, String> {
    let parts: Vec<&str> = expr.split_whitespace().collect();

    if parts.len() != 3 {
        return Err("Input should be in the format 'a op b'".to_string());
    }

    let a: i32 = parts[0].parse().map_err(|_| "Invalid number".to_string())?;
    let b: i32 = parts[2].parse().map_err(|_| "Invalid number".to_string())?;

    let op = parts[1].trim();

    match op {
        "+" => Ok(a + b),
        "-" => Ok(a - b),
        "*" => Ok(a * b),
        "/" => safe_division(a, b),
        _ => Err("Unsupported operator".to_string()),
    }
}

fn main() {
    loop {
        
        println!(">");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            eprintln!("Error reading input");
            continue;
        }

        let input = input.trim();
        if input.eq_ignore_ascii_case("exit") {
            println!("Exiting the program.");
            break;
        }

        match evaluate_expression(input) {
            Ok(result) => println!("Result: {}", result),
            Err(e) => eprintln!("Error: {}", e),
        }
    }
}
