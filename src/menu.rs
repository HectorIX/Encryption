extern crate rand;

use self::rand::{ Rng, OsRng };


use std::io::{self, BufRead};
use std;
use fileIO;
use menu;
use aes256;
use sha256;
use caesar_cipher_ascii;



// Print out the menu
fn print_menu() {

    println!("\n%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%\n");
    println!("\t 1. Store your plaintext message in a file.");
    println!("\t 2. Encrypt the plaintext message using Caesar Cipher.");
    println!("\t 3. Decrypt the ciphertext [Caesar Cipher decryptor].");
    println!("\t 4. Encrypt the plaintext message using AES-256.");
    println!("\t 5. Decrypt the ciphertext [AES-256 decryptor].");
    println!("\t 6. Calculate the SHA-256 signature of a file.");
    println!("\t 7. Exit Program.");
    println!("\n%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%\n");
}


// Ask user to insert a value from the keyboard
// given the menu
// and depending the user's choice
// an action is performed.
pub fn menu() {


    let mut message = String::new();
    let mut encrypted_message = String::new();
    let mut plaintext_message = String::new();

    let mut userChoice:i32 = 0;
    let mut dencryption_key:i32;

    // Filenames
    let inputFile: String = "input.txt".to_string();
    let caesarInput: String = "caesarInput.txt".to_string();
    let caesarOutput: String = "caesarOutput.txt".to_string();
    let aesInput: String = "aesInput.txt".to_string();
    let aesOutput: String = "aesOutput.txt".to_string();
    let initializedVector: String = "iv.txt".to_string();


    // while the user's choice is not 7 (Exit)
    // the loop is continuously executed.
    while userChoice != 7 {

        print_menu();
        println!("Your choice: " );
        userChoice = read_integer();


        match userChoice {

            1 => {
                    // Case 1:
                    // The user inserts a message using the keyboard.
                    println!("Insert a plaintext message: ");
                    message = read_message_stdin();

                    // Store this message to the specified files.
                    fileIO::write_file(inputFile.clone(),message.clone());
                    fileIO::write_file(caesarInput.clone(),message.clone());
                    fileIO::write_file(aesInput.clone(),message.clone());
                 },
            2 => {
                    // Case 2:
                    // Read the "caesarInput.txt" file.
                    // Encrypt this message using a key given by the user.
                    // And store the encrypted message to the "caesarOutput.txt".

                    // Read the message from "caesarInput.txt"
                    message = fileIO::read_file(caesarInput.clone());

                    // Encrypt the message.
                    encrypted_message = caesar_cipher_ascii::encrypt_ascii(message);
                    println!("The encrypted message is: {}", encrypted_message );

                    // Store the message to the "caesarOutput.txt"
                    fileIO::write_file(caesarOutput.clone(),encrypted_message)
                },
            3 => {
                    // Case 3:
                    // Read the "caesarOutput.txt" file.
                    // Then ask for the decryption key.
                    // Use the encrypted message alongside with the decryption key
                    // and decrypt the message.
                    // Then print out the decrypted message.
                    // ( If user provide a wrong key, the printed message will not be readable. )


                    // Read from "caesarOutput.txt"
                    encrypted_message = fileIO::read_file(caesarOutput.clone());

                    // Ask for the decryption key.
                    println!("Enter the decryption key (a positive integer):" );
                    dencryption_key = menu::read_integer() % 96;

                    // Decrypt the message and print it out.
                    plaintext_message = caesar_cipher_ascii::decrypt_ascii(encrypted_message, dencryption_key);
                    println!("The plaintext message is: {:?}", plaintext_message)
                 },
            4 => {
                    let mut key: [u8; 32] = [0; 32];
                    let mut iv: [u8; 16] = [0; 16];

                    // Read the message from "aesInput.txt"
                    message = fileIO::read_file(aesInput.clone());


                    let mut rng = OsRng::new().ok().unwrap();
                    rng.fill_bytes(&mut key);
                    rng.fill_bytes(&mut iv);

                    // Encrypt the message using AES-256 encryption algorithm.
                    let var = aes256::aes256_encrypt(&message.clone().into_bytes(), &key, &iv).ok().unwrap();

                    let mut array: String = "".to_string();
                    for letter in var {
                        array.push(std::char::from_u32(letter as u32).unwrap());
                    }


                    // Store the message to the "aesOutput.txt"
                    fileIO::write_file(aesOutput.clone(), array);
                    println!("Your message is secure!");

                    let message2 = fileIO::read_file(aesOutput.clone());
                    let mut vec: Vec<u8> = Vec::new();

                    for value in message2.chars() {
                        vec.push(value as u8);
                    }


                    let var2 = aes256::aes256_decrypt(&vec[..], &key, &iv);


                    println!("{:?}", var2.unwrap());
                 },
            5 => {


                 },
            6 => {


                 },
            7 => {
                    println!("Program terminated!");
                    break
                 },
            _ => println!("Typo error! Please try again..."),
        }

    }

}



// User inserts an integer value
// from the standart input (keyboard)
// and return this value.
pub fn read_integer() -> i32 {

    let mut choice = 0;
    let mut input_text = String::new();


        io::stdin()
            .read_line(&mut input_text)
            .expect("failed to read from stdin");

        let trimmed = input_text.trim();
        match trimmed.parse::<i32>() {
            Ok(i) => choice = i,
            Err(..) => println!("this was not an integer: {}", trimmed)
        };

    choice

}


// User insterts a message (one line only is permitted)
// using the the standart input (keyboard)
// and return this message.
fn read_message_stdin() -> String {

    let mut message = String::new();
    let stdin = io::stdin();

    println!("Enter your message:" );
    stdin.lock().read_line(&mut message)
                .expect("Could not read your message");

    message

}
