use magic_crypt::{new_magic_crypt, MagicCryptTrait};

//DESCRIPTAR
pub fn decripto(r: String) -> String{

    let mc = new_magic_crypt!(r, 256);
    let base64 = mc.encrypt_str_to_base64("pablo");

    mc.decrypt_base64_to_string(&base64).unwrap()
}

