use magic_crypt::{new_magic_crypt, MagicCryptTrait};

//ENCRIPTAR
pub fn cripto(r: String) -> String{

    let mc = new_magic_crypt!(r, 256);
    let base64 = mc.encrypt_str_to_base64("pablo");

    base64
}

