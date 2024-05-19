use std::process::Command;
use std::io::{self, BufRead};
use std::env;

fn main() -> io::Result<()> {
    // Get the keyword from command-line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <keyword>", args[0]);
        std::process::exit(1);
    }
    let keyword = &args[1];

    // Run the Go program with the -l jp argument and the search keyword
    let golang_output = Command::new("/usr/local/bin/angel")
        .arg("-l")
        .arg("jp")
        .arg(keyword)
        .output()
        .expect("Failed to run Go program");
    // Capture the output and convert it to a string
    let output = String::from_utf8(golang_output.stdout).expect("Failed to parse output");

    // Read the output line by line
    let reader = io::BufReader::new(output.as_bytes());

    for line in reader.lines() {
        let line = line?;
        
        // Extract magnet link using a regular expression
        let re = regex::Regex::new(r"magnet:\?xt=urn:btih:[a-zA-Z0-9]{40}").expect("Failed to compile regex");
        if let Some(magnet_link) = re.find(&line) {
            // Open the magnet link with qBittorrent
            Command::new("open")
                .arg("-a")
                .arg("qbittorrent")
                .arg(magnet_link.as_str())
                .output()
                .expect("Failed to open qBittorrent with magnet link");
        }
    }

    Ok(())
}
