enum Command {
    Add(String),
    Remove(String),
    Quit,
}

fn parse_command(parts: &[&str]) -> Option<Command> {
    match parts {
        ["add", item] => Some(Command::Add(item.to_string())),
        ["remove", item] => Some(Command::Remove(item.to_string())),
        ["quit"] => Some(Command::Quit),
        _ => None,
    }
}

fn main() {
    let stdin = std::io::stdin();

    loop {
        let mut input = String::new();
        stdin.read_line(&mut input).expect("Failed to read line");

        let parts = input.split_whitespace().collect::<Vec<&str>>();
        println!("You entered: {:?}", parts);
        if let Some(cmd) = parse_command(&parts) {
            match cmd {
                Command::Add(item) => println!("Adding item: {}", item),
                Command::Remove(item) => println!("Removing item: {}", item),
                Command::Quit => {
                    println!("Exiting...");
                    break;
                }
            }
        } else {
            println!("Unknown command. Please try again.");
        }
    }
}
