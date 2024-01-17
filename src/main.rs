use std::{fs::{self, File}, io::{self, BufRead}};

use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHasher, PasswordVerifier, PasswordHash,
};

fn main() {
    let hash = read_hash_file();
    read_word_list(hash);
}


fn read_hash_file() -> String {
    let contents = fs::read_to_string("./hash.txt").unwrap();
    return contents;
}

fn read_word_list(hash: String) {
    let file = File::open("./word-list.txt").unwrap();
    let reader = io::BufReader::new(file);

    let argon2 = Argon2::default();
    for line in reader.lines() {
        let string_password = line.unwrap();
        let salt = SaltString::generate(&mut OsRng);
        let hashed_password = PasswordHash::new(&hash).unwrap();
        let bool = argon2.verify_password(string_password.as_bytes(), &hashed_password).is_ok();
        if bool == true {
            println!("The password is: ");
            println!("{:?}", string_password);
            return;
        }
    }
    println!("No Password Matches Found.")
}