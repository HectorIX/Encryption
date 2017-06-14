extern crate crypto;

use crypto::digest::Digest;
use crypto::sha2::Sha256;


pub fn sha256( input: String ) -> output:String {

    let mut sha256 = Sha256::new();
    let output = sha256.input_str(input);

    Ok(output)
}
