use std::io::{self, BufRead};
use fileIO;
use menu;
//use caesar_cipher;
use caesar_cipher_ascii;



// Ask user to insert a value from the keyboard
// given the menu
// and depending the user's choice
// an action is performed.
pub fn menu() {


    let mut message = String::new();
    let mut encrypted_message = String::new();
    let mut plaintext_message = String::new();

    let mut choice:i32 = 0;
    let mut dencryption_key:i32;


    // while the user's choice is not 4 (Exit)
    // the loop is continuously executed.
    while choice != 4 {

        print_menu();
        println!("Your choice: " );
        choice = read_integer();


        match choice {

            1 => {
                    // Case 1:
                    // The user inserts a message using the keyboard.
                    // And this message is stored into the "input.txt" file.
                    let write_filename:String = "input.txt".to_string();
                    println!("Insert a plaintext message: ");
                    message = read_message_stdin();
                    fileIO::write_file(write_filename,message)
                 },
            2 => {
                    // Case 2:
                    // Read the "input.txt" file as message.
                    // Encrypt this message using a key given by the user.
                    // And store the encrypted message to the "output.txt".

                    let read_filename:String = "input.txt".to_string();

                    // Read the message from "input.txt"
                    message = fileIO::read_file(read_filename);

                    // Encrypt the message.
                    encrypted_message = caesar_cipher_ascii::encrypt_ascii(message);
                    println!("The encrypted message is: {}", encrypted_message );

                    // Store the message to the "output.txt"
                    let write_filename:String = "output.txt".to_string();
                    fileIO::write_file(write_filename,encrypted_message)
                },
            3 => {
                    // Case 3:
                    // Read "output.txt" as encrypted message.
                    // Then ask for the decryption key.
                    // Use the encrypted message alongside with the decryption key
                    // and decrypt the message.
                    // Then print out the decrypted message.
                    // (If user provide a wrong key, the printed message will not be readable. )

                    let read_filename:String = "output.txt".to_string();

                    // Read from "output.txt"
                    encrypted_message = fileIO::read_file(read_filename);

                    // Ask for the decryption key.
                    println!("Enter the decryption key (a positive integer):" );
                    dencryption_key = menu::read_integer() % 96;

                    // Decrypt the message and print it out.
                    plaintext_message = caesar_cipher_ascii::decrypt_ascii(encrypted_message, dencryption_key);
                    println!("The plaintext message is: {:?}", plaintext_message)
                 },
            4 => {
                    println!("Program terminated!");
                    break
                 },
            _ => println!("Typo error! Please try again..."),
        }

    }

}


// Print out the menu
fn print_menu() {

    println!("\n%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%\n");
    println!("\t 1. Store your plaintext message in a file.");
    println!("\t 2. Encrypt the plaintext message.");
    println!("\t 3. Decrypt the plaintext message.");
    println!("\t 4. Exit Program.");
    println!("\n%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%\n");
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
