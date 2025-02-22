use std::io::{BufRead, BufReader};
use std::process::{Child, Command, Stdio};

pub struct ServerGuard {
    child: Child,
    port: u16,
}

impl ServerGuard {
    pub fn new() -> Self {
        let port = 8080; // Default port set by `cargo component serve`

        // Cleanup any existing process
        Self::kill_process_by_port(port);

        // Start the server with piped stdout
        let mut child = Command::new("cargo")
            .arg("component")
            .arg("serve")
            .stderr(Stdio::piped())
            .spawn()
            .expect("Failed to execute `cargo component serve`");

        // Get stdout handle and create a reader
        let stderr = child.stderr.take().expect("Failed to capture stdout");
        let mut reader = BufReader::new(stderr);

        // Read lines until we see the listening message
        for line in reader.lines() {
            let line = line.expect("Failed to read line");
            print!("{}", line); // Forward output
            if line.contains("Serving HTTP") {
                break;
            }
        }

        // Set stdout to inherit for remaining output
        child.stdout = None;

        ServerGuard { child, port }
    }

    fn kill_process_by_port(port: u16) {
        let output = Command::new("lsof")
            .args(["-i", &format!("tcp:{}", port), "-t"])
            .output()
            .expect("Failed to execute lsof");

        if let Ok(pid) = String::from_utf8(output.stdout) {
            if !pid.trim().is_empty() {
                Command::new("kill")
                    .arg("-9")
                    .arg(pid.trim())
                    .output()
                    .expect("Failed to kill process");
            }
        }
    }
}

impl Drop for ServerGuard {
    fn drop(&mut self) {
        Self::kill_process_by_port(self.port);
        self.child.wait().expect("Failed to wait for child process");
    }
}
