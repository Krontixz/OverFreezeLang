#[path = "../../Logic/install_logic.rs"]
mod install_logic;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 3 {
        println!("OVFZ_CARGO: Usage -> cargo install overfreeze");
        return;
    }

    // Check for "install overfreeze"
    if args == "install" && args == "overfreeze" {
        install_logic::perform_installation();
    } else {
        println!("OVFZ_CARGO: Unknown command sequence.");
    }
}
