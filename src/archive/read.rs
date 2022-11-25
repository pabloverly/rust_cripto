use std::fs::File;
use std::io::prelude::*;

pub fn read()-> std::io::Result<()>{
    let mut files = File::open("./src/data/foo.bin")?;
    let mut contents = String::new();
    files.read_to_string(&mut contents)?;
    println!("{contents}");
    //assert_eq!(contents, "Hello, world!");
    Ok(())
}