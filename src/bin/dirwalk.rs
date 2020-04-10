// Adapt this function to return a vector of strings for which the bool is true,
// rather than return a Vec<bool> directly.

// original question by ImportantString#5154 on discord

use std::fs;
use std::io;

fn get_block_devices() -> Result<Vec<bool>, io::Error> {
    fs::read_dir("/sys/block")?
        .map(|res| res.map(|e| e.file_name()))
        .map(|res| {
            res.map(|e| {
                let path = e.into_string().unwrap();
                !path.contains(&"loop") && path != "sr0"
            })
        })
        .collect::<Result<Vec<bool>, io::Error>>()
}

fn main() {
    println!("{:#?}", get_block_devices());
}
