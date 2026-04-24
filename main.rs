#[path = "fileTypes/main/of_file.rs"]
mod of_file;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        return;
    }

    let target = &args;

    if target.ends_with(".of") {
        of_file::LogicController::init(target);
    }
}
