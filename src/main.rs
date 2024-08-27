use rand::Rng;
use rand::seq::SliceRandom;
use std::fs::OpenOptions;
use std::io::{self, Write};

fn generate_password(length: usize, use_uppercase: bool, use_lowercase: bool, use_numbers: bool, use_special: bool) -> String {
    let mut rng = rand::thread_rng();
    let mut all_chars = Vec::new();

    if use_uppercase {
        all_chars.extend('A'..='Z');
    }
    if use_lowercase {
        all_chars.extend('a'..='z');
    }
    if use_numbers {
        all_chars.extend('0'..='9');
    }
    if use_special {
        all_chars.extend(vec!['!', '@', '#', '$', '%', '^', '&', '*', '(', ')', '-', '_', '=', '+']);
    }

    (0..length)
        .map(|_| *all_chars.choose(&mut rng).expect("Karakter seti boş olamaz!"))
        .collect()
}

fn check_password_strength(password: &str) -> &str {
    let length = password.len();
    let has_uppercase = password.chars().any(|c| c.is_uppercase());
    let has_lowercase = password.chars().any(|c| c.is_lowercase());
    let has_numbers = password.chars().any(|c| c.is_numeric());
    let has_special_chars = password.chars().any(|c| !c.is_alphanumeric());

    match (length, has_uppercase, has_lowercase, has_numbers, has_special_chars) {
        (l, true, true, true, true) if l >= 12 => "Very Strong",
        (l, true, true, true, true) if l >= 8 => "Strong",
        (l, true, true, true, _) if l >= 8 => "Moderate",
        _ => "Weak",
    }
}

fn save_password_to_file(password: &str) -> io::Result<()> {
    let mut file = OpenOptions::new().append(true).open("passwords.txt")?;
    writeln!(file, "{}", password)?;
    Ok(())
}

fn main() {
    let mut length_input = String::new();
    println!("Şifre uzunluğunu girin:");
    io::stdin().read_line(&mut length_input).expect("Okuma hatası");
    let length: usize = length_input.trim().parse().expect("Geçerli bir sayı girin.");

    println!("Büyük harf kullanılsın mı? (y/n):");
    let mut uppercase_input = String::new();
    io::stdin().read_line(&mut uppercase_input).expect("Okuma hatası");
    let use_uppercase = uppercase_input.trim().eq_ignore_ascii_case("y");

    println!("Küçük harf kullanılsın mı? (y/n):");
    let mut lowercase_input = String::new();
    io::stdin().read_line(&mut lowercase_input).expect("Okuma hatası");
    let use_lowercase = lowercase_input.trim().eq_ignore_ascii_case("y");

    println!("Rakamlar kullanılsın mı? (y/n):");
    let mut numbers_input = String::new();
    io::stdin().read_line(&mut numbers_input).expect("Okuma hatası");
    let use_numbers = numbers_input.trim().eq_ignore_ascii_case("y");

    println!("Özel karakterler kullanılsın mı? (y/n):");
    let mut special_chars_input = String::new();
    io::stdin().read_line(&mut special_chars_input).expect("Okuma hatası");
    let use_special = special_chars_input.trim().eq_ignore_ascii_case("y");

    println!("Kaç adet şifre oluşturulsun?");
    let mut count_input = String::new();
    io::stdin().read_line(&mut count_input).expect("Okuma hatası");
    let count: usize = count_input.trim().parse().expect("Geçerli bir sayı girin.");

    for _ in 0..count {
        let password = generate_password(length, use_uppercase, use_lowercase, use_numbers, use_special);
        let strength = check_password_strength(&password);
        println!("Generated password: {}\nStrength: {}", password, strength);

        save_password_to_file(&password).expect("Şifre dosyaya kaydedilemedi.");
    }

    println!("Şifreler passwords.txt dosyasına kaydedildi.");
}
