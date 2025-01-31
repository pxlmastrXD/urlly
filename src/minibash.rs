use std::process::Command;

// Tested 1/31/25
pub fn runbash(x: &str) -> String {
    let command = "bash";
    let argument = "-c";
    let output = Command::new(command)
        .arg(argument)
        .arg(x)
        .output()
        .expect("Failed to execute command");

    let output_bytes = output.stdout;
    let output_string = String::from_utf8_lossy(&output_bytes);
    
    output_string.to_string()
}