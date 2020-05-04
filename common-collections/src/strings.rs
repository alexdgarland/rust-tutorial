
fn get_demo_strings() -> Vec<String> {
    vec![
        "السلام عليكم",
        "Dobrý den",
        "Hello",
        "שָׁלוֹם",
        "नमस्ते",
        "こんにちは",
        "안녕하세요",
        "你好",
        "Olá",
        "Здравствуйте",
        "Hola"
    ].iter().map(|str| str.to_string()).collect()
}

fn show_string_char_byte_details(string: &String) {
    println!("\nGreeting - \"{}\" - {} characters, {} bytes.", string, string.chars().count(), string.len());
    println!("{:12}{:12}{}", "Character", "No. Bytes", "Bytes");
    let bytes = string.as_bytes();
    let mut offset = 0;
    for c in string.chars() {
        let char_len = c.len_utf8();
        println!("{:12}{:<12}{:?}", c, char_len, &bytes[offset..(offset + (char_len))]);
        offset += char_len;
    };
}

pub fn demo_strings() {
    for s in get_demo_strings().iter() {
        show_string_char_byte_details(s);
    }
}