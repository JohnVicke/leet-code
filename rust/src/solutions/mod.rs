mod palindrome_number;
mod two_sum;

fn clear_console() {
    print!("\x1B[2J\x1B[1;1H");
    std::io::Write::flush(&mut std::io::stdout()).unwrap();
}

pub fn choose_solution() {
    loop {
        clear_console();
        let mut input = String::new();
        println!("Please enter the number of the problem you want to solve:");
        println!("x for exit");
        println!("1. Two Sum");
        println!("2. Palindrome Number");
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match input.trim() {
            "1" => println!("You chose Two Sum"),
            "2" => println!("You chose Palindrome Number"),
            "x" => break,
            _ => println!("Please enter a valid number"),
        }
        std::thread::sleep(std::time::Duration::from_secs(2));
    }
}
