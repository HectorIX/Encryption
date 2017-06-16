extern crate crypto;

use self::crypto::digest::Digest;
use self::crypto::sha2::Sha256;


pub fn sha256( input: String ) -> String {

    let mut sha256 = Sha256::new();

    sha256.input_str(input.as_str());
    let sha256_value = sha256.result_str();

    sha256_value
}
