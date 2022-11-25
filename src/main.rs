mod generation;
use generation::cripto::*;
use generation::decripto::*;

mod archive;
use archive::read::*;
use archive::write::*;

fn main()-> std::io::Result<()>{ 
    let infor =  cripto("estoque".to_string());

    //println!("{}", decripto("MAYyjozzJJQV1ZD35kQyaw==".to_string()));
    write(infor); 
    read();    
    Ok(())
}

