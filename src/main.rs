use rand::{Rng, thread_rng};

fn main() {
    let password_length: u8 = 15;
    let password = generate_password(password_length);
    println!("Generated Password: {}", password);
}

fn generate_password(password_length: u8) -> String {
    let mut result = String::new();
    for _ in 0..password_length {
        let number: u32 = thread_rng().gen_range(48..122);
        let characters: char = std::char::from_u32(number).unwrap();
        result.push(characters);
    }
    result
}