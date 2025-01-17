/*
PixelDB
Copyright (C) Caleb Wietholter, 2024. Licensed under the MIT License.
*/
use std::fs::File;
mod minibash {
    use std::process::Command;
    fn runbash(command: &str) -> &str {
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
}


// Primary Database operations as a Struct
struct Database {
    db: &str
}

// Operations
impl Database {
    pub fn set(&mut self, key: &str, value: &str) -> &str {
        // Create new file
        let mkdiroutput = minibash::runbash(format!("mkdir {self.db}/{key}"));
        match mkdiroutput {
            "error" => {panic!("Failed to create directory")},
            else => {println!();}
        };

        let touchoutput = minibash::runbash(format!("touch {self.db}/{key}/key"));
        match touchoutput {
            "error" => panic!("Failed to create keyfile"),
            else => println!()
        };

        let writeoutput = minibash::runbash(format!("echo {value} > {self.db}/{key}/key"));
        match writeoutput {
            "error" => panic!("Failed to write to keyfile"),
            else => println!()
        }
        return key
    }

    pub fn get(&mut self, key: &str) -> &str {
        let output = minibash::runbash(format!("cat {self.db}/{key}/key"));
        match output {
            "error" => {panic!("Failed to read key")},
            else => println!()
        }
        return output
    } 
}
