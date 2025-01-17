use std::process::Command;

pub fn runbash(command: &str) -> &str {
    let output = Command::new("bash")
        .arg("-c")
        .arg(command)
        .output();

    if let Ok(output) = output {
        if output.status.success() {
            return Box::leak(String::from_utf8_lossy(&output.stdout).trim().to_string().into_boxed_str());
        }
    }
    "error"
}

pub fn checkOutput(output: &str) -> bool {
    return match output {
        "error" => false,
        else => true
    }
}