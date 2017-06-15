use menu;
use std;


// Encrypt the message using the Caesar Cipher's Algorithm
// and return the encrypted message as a string.
pub fn encrypt_ascii(message:String) -> String {

    let encryption_key:i32;
    let mut encrypted_message = String::new();

    // Ask user to provide the encryption key.
    println!("Enter the encryption key (a positive integer):" );
    encryption_key = menu::read_integer() % 96;

    // Shift each character or the plaintext message to the right
    // as many positions as are specified by the encryption key.
    for character in message.chars() {

        //this will hold the character we are adding to the ciphered message
        let new_char;
        let mut my_ch = character as u32;

        if (my_ch >= 32) & (my_ch < 127) {

            my_ch += encryption_key as u32;

            if my_ch > '~' as u32 {
                my_ch -= 96;
            }
        }
        else {
            my_ch += 0;
        }

        new_char = std::char::from_u32(my_ch).unwrap();

        //push each encrypted character to the encrypted_message
        encrypted_message.push(new_char);
    }

    // return the encrypted message.
    encrypted_message

}


// Decrypt the message using the encryption key
// and return the plaintext message as a string.
pub fn decrypt_ascii( encrypted_message: String, encryption_key:i32 ) -> String {


    let mut plaintext_message = String::new();


    for character in encrypted_message.chars() {
        //this will hold the character we are adding to the ciphered message
        let new_char;
        let mut my_ch = character as u32;

        if (my_ch >= 32) & (my_ch < 127) {

            my_ch -= encryption_key as u32;

            if my_ch < ' ' as u32 {
                my_ch += 96;
            }
        }
        else {
            my_ch += 0;
        }

        new_char = std::char::from_u32(my_ch).unwrap();

        //push each decrypted character to the plaintext_message
        plaintext_message.push(new_char);
    }

    // return the plaintext message
    plaintext_message
}
