use std::process::Command;
use std::env;
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();

    // note that there are a number of downsides to this approach, the comments
    // below detail how to improve the portability of these commands.
    Command::new("cargo").args(&["install", "spot-client"])
                       .status().unwrap();
    Command::new("cargo").args(&["install", "spot-server"])
                       .status().unwrap();GIT GUI
                       
}