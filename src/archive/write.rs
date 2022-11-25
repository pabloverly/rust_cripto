use std::fs::File;
use std::io::prelude::*;


pub fn write(info: String)-> std::io::Result<()>{
    // let valor = cripto("pablo".to_string());   
    let valor = info.to_string(); 
    let mut file = File::create("foo.bin")?;
    file.write_all(valor.as_bytes());         
    Ok(())
}
