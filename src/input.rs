use crate::prelude::*;

pub fn get_target_input() -> String {
     let mut target = String::new();
     println!("Enter the ip address or website to scan:");
    io::stdin()
        .read_line(&mut target)
        .expect("Failed to read line");

    target.trim().to_string()

}

pub fn get_mode_input() -> Mode {
    let mut mode_input = String::new();
    println!("Enter the mode (full/fast):");
    io::stdin()
        .read_line(&mut mode_input)
        .expect("Failed to read line");
   let mode_input = mode_input.trim().to_ascii_lowercase();
    let mode = match mode_input.as_str() {
        "full" => Mode::Full,
        "fast" => Mode::Fast,
        _ => {
            println!("Invalid mode selected, defaulting to Fast mode");
            Mode::Fast
        }
    };
    mode
}
