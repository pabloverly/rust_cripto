use magic_crypt::{new_magic_crypt, MagicCryptTrait};

mod generation;
use generation::cripto::*;
use generation::decripto::*;

fn main(){   
    println!("{}", cripto("pablo".to_string()));
    println!("{}", decripto("MAYyjozzJJQV1ZD35kQyaw==".to_string()));
}

