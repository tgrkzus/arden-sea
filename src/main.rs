use std::io;
use std::io::Write;

fn main() {

    // Init game
    loop {
        // Game loop
        println!("1) Play");
        println!("2) Options");
        println!("3) Quit");
        print!("Input: ");
        io::stdout().flush();
        let choice = get_choice();
        print!("{}", choice);

        // Clear terminal
        print!("{}[2J", 27 as char);
    }

    // Cleanup
}

fn get_choice() -> String {
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("failed");
    return choice;
}
