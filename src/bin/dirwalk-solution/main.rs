use std::fs;
use std::io;

fn get_block_devices() -> Result<Vec<String>, io::Error> {
    fs::read_dir("/sys/block")?
        .map(|res| res.map(|e| e.file_name().into_string().unwrap()))
        .filter_map(|res| {
            res.map(|path| {
                if !path.contains(&"loop") && path != "sr0" {
                    Some(path)
                } else {
                    None
                }
            })
            // Result<Option<String>, io::Error> → Option<Result<String, io::Error>>
            .transpose()
        })
        .collect()
}

fn main() {
    println!("{:#?}", get_block_devices());
}
