use swaystatus::Module;

use std::process::Command;

pub struct Updates {
    update_count: usize,
    call_count: u16,
}

impl Updates {
    pub fn new() -> Self {
        Updates {
            update_count: 0,
            call_count: 0,
        }
    }
}

impl Module for Updates {
    fn value(&self) -> String {
        self.update_count.to_string()
    }

    fn update(&mut self) {
        // only really update internal value after n function calls
        if self.call_count == 150 {
            self.call_count = 0;

            // call the Arch Linux provided python script which prints each update on a new line
            let output = match Command::new("checkupdates").output() {
                Ok(output) => output,
                Err(err) => {
                    eprintln!("Executing the command failed: {}", err);
                    return;
                }
            };

            if !output.status.success() {
                eprintln!("Command executed with failing error code");
                return;
            }

            self.update_count = match String::from_utf8(output.stdout) {
                Ok(string) => string.lines().count(),
                Err(err) => {
                    eprintln!("Unable to create string from stdout: {}", err);
                    return;
                }
            };
        } else {
            self.call_count += 1;
        }
    }
}
