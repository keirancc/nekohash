use nekohash::{
    KawaiiHash, MagicalHash, TsundereHash, NekoHash,
    utils::{
        encrypt_data, decrypt_data, generate_key, key_to_base64,
        to_hex, combine_hashes, constant_time_compare,
        stretch_key, derive_key, generate_salt, time_based_key, rotate_key
    }
};
use std::io::{self, Write};
use colored::*;
use dialoguer::{theme::ColorfulTheme, Select, Input};

fn print_banner() {
    println!("\n{}", "=".repeat(80).bright_magenta());
    println!("{}", "🐱 Welcome to NekoHash Demo 🐱".bright_cyan().bold());
    println!("{}", "A Kawaii Cryptographic Hashing Library".bright_cyan());
    println!("{}", "=".repeat(80).bright_magenta());
}

fn print_section(title: &str) {
    println!("\n{}", "~".repeat(40).bright_blue());
    println!("{}", title.bright_yellow().bold());
    println!("{}", "~".repeat(40).bright_blue());
}

fn pause() {
    print!("\nPress Enter to continue...");
    io::stdout().flush().unwrap();
    let mut _input = String::new();
    io::stdin().read_line(&mut _input).unwrap();
}

fn demo_hash_algorithms() {
    print_section("📝 Hash Algorithm Demonstration");
    
    let input = "Hello, Neko World!";
    println!("\nInput text: {}", input.bright_green());

    // KawaiiHash Demo
    println!("\n{}:", "🌸 KawaiiHash".bright_magenta());
    let kawaii = KawaiiHash::new();
    let kawaii_hash = kawaii.hash(input.as_bytes());
    println!("Default (32 bytes): {}", to_hex(&kawaii_hash).bright_cyan());

    let kawaii_16 = KawaiiHash::with_size(16);
    let kawaii_hash_16 = kawaii_16.hash(input.as_bytes());
    println!("Custom size (16 bytes): {}", to_hex(&kawaii_hash_16).bright_cyan());

    // MagicalHash Demo
    println!("\n{}:", "✨ MagicalHash".bright_magenta());
    let magical = MagicalHash::new();
    let magical_hash = magical.hash(input.as_bytes());
    println!("Default: {}", to_hex(&magical_hash).bright_cyan());

    let magical_custom = MagicalHash::with_magic(0xCAFEBABE);
    let magical_hash_custom = magical_custom.hash(input.as_bytes());
    println!("Custom magic: {}", to_hex(&magical_hash_custom).bright_cyan());

    // TsundereHash Demo
    println!("\n{}:", "💕 TsundereHash".bright_magenta());
    let tsundere = TsundereHash::new();
    let tsundere_hash = tsundere.hash(input.as_bytes());
    println!("Default (8 rounds): {}", to_hex(&tsundere_hash).bright_cyan());

    let tsundere_16 = TsundereHash::with_rounds(16);
    let tsundere_hash_16 = tsundere_16.hash(input.as_bytes());
    println!("16 rounds: {}", to_hex(&tsundere_hash_16).bright_cyan());

    pause();
}

fn demo_encryption() {
    print_section("🔐 Encryption Features");

    let data = "Secret Neko Message";
    println!("\nOriginal data: {}", data.bright_green());

    // Generate a key
    let key = generate_key();
    let key_b64 = key_to_base64(&key);
    println!("\nGenerated key (base64): {}", key_b64.bright_cyan());

    // Encrypt the data
    let encrypted = encrypt_data(data.as_bytes(), Some(&key)).unwrap();
    println!("Encrypted (base64): {}", String::from_utf8_lossy(&encrypted).bright_cyan());

    // Decrypt the data
    let decrypted = decrypt_data(&encrypted, &key).unwrap();
    println!("Decrypted: {}", String::from_utf8(decrypted).unwrap().bright_green());

    pause();
}

fn demo_key_derivation() {
    print_section("🔑 Key Derivation Features");

    let password = "MySecretPassword123";
    println!("\nPassword: {}", password.bright_green());

    // Generate a salt
    let salt = generate_salt();
    println!("Generated salt: {}", to_hex(&salt).bright_cyan());

    // Derive a key
    let derived_key = derive_key(password.as_bytes(), &salt).unwrap();
    println!("Derived key: {}", to_hex(&derived_key).bright_cyan());

    // Key stretching
    let stretched_key = stretch_key(password.as_bytes(), 10000, 32).unwrap();
    println!("\nStretched key (10000 iterations): {}", to_hex(&stretched_key).bright_cyan());

    // Time-based key
    let time_key = time_based_key(password.as_bytes(), 30).unwrap();
    println!("Time-based key (30s window): {}", to_hex(&time_key).bright_cyan());

    pause();
}

fn demo_utility_functions() {
    print_section("🛠️ Utility Functions");

    // Combine hashes
    let hash1 = vec![1, 2, 3, 4];
    let hash2 = vec![5, 6, 7, 8];
    let combined = combine_hashes(&[hash1.clone(), hash2.clone()]);
    println!("\nCombining hashes:");
    println!("Hash 1: {}", to_hex(&hash1).bright_cyan());
    println!("Hash 2: {}", to_hex(&hash2).bright_cyan());
    println!("Combined: {}", to_hex(&combined).bright_cyan());

    // Constant-time comparison
    let result = constant_time_compare(&hash1, &hash1);
    println!("\nConstant-time comparison (same hashes): {}", result.to_string().bright_green());
    let result = constant_time_compare(&hash1, &hash2);
    println!("Constant-time comparison (different hashes): {}", result.to_string().bright_red());

    // Key rotation
    let key = generate_key();
    println!("\nOriginal key: {}", to_hex(&key).bright_cyan());
    let rotated = rotate_key(&key, 8);
    println!("Rotated key (8 bits): {}", to_hex(&rotated).bright_cyan());

    pause();
}

fn interactive_demo() {
    print_section("🎮 Interactive Demo");

    let theme = ColorfulTheme::default();
    let items = vec!["KawaiiHash", "MagicalHash", "TsundereHash"];
    
    let selection = Select::with_theme(&theme)
        .with_prompt("Select a hash algorithm")
        .items(&items)
        .default(0)
        .interact()
        .unwrap();

    let input: String = Input::with_theme(&theme)
        .with_prompt("Enter text to hash")
        .default("Hello, Neko World!".into())
        .interact_text()
        .unwrap();

    let hash = match selection {
        0 => {
            let size: usize = Input::with_theme(&theme)
                .with_prompt("Enter output size (bytes)")
                .default(32)
                .interact_text()
                .unwrap();
            KawaiiHash::with_size(size).hash(input.as_bytes())
        },
        1 => {
            let magic: u32 = Input::with_theme(&theme)
                .with_prompt("Enter magic number (hex)")
                .default(0x19950816)
                .interact_text()
                .unwrap();
            MagicalHash::with_magic(magic).hash(input.as_bytes())
        },
        2 => {
            let rounds: usize = Input::with_theme(&theme)
                .with_prompt("Enter number of rounds")
                .default(8)
                .interact_text()
                .unwrap();
            TsundereHash::with_rounds(rounds).hash(input.as_bytes())
        },
        _ => unreachable!()
    };

    println!("\nInput: {}", input.bright_green());
    println!("Hash: {}", to_hex(&hash).bright_cyan());

    pause();
}

fn main() {
    print_banner();

    if !dialoguer::console::user_attended() {
        println!("\n{}", "⚠️ This demo requires an interactive terminal!".bright_red());
        println!("{}", "Please run it in a terminal environment.".bright_yellow());
        return;
    }

    loop {
        println!("\n{}", "Menu:".bright_yellow().bold());
        println!("1. 📝 Hash Algorithm Demo");
        println!("2. 🔐 Encryption Demo");
        println!("3. 🔑 Key Derivation Demo");
        println!("4. 🛠️ Utility Functions Demo");
        println!("5. 🎮 Interactive Demo");
        println!("6. 🚪 Exit");

        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Choose a demo")
            .items(&["Hash Algorithms", "Encryption", "Key Derivation", "Utility Functions", "Interactive Demo", "Exit"])
            .default(0)
            .interact()
            .unwrap();

        match selection {
            0 => demo_hash_algorithms(),
            1 => demo_encryption(),
            2 => demo_key_derivation(),
            3 => demo_utility_functions(),
            4 => interactive_demo(),
            5 => break,
            _ => unreachable!()
        }
    }

    println!("\n{}", "Thank you for using NekoHash! 🐱".bright_cyan().bold());
}
