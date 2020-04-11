use std::io;

fn main() {
    let mut text: String = String::new();

    println!("Input some text for convertion to pig latin!");

    io::stdin()
        .read_line(&mut text)
        .expect("Failed to read line");

    println!("{}", pig_latin(text));
}

fn pig_latin(text: String) -> String {
    let mut updated_text = String::new();
    let text = text.trim();

    for word in text.split_whitespace() {
        let first_letter = &word[0..1].to_lowercase()[..];
        if first_letter == "a"
            || first_letter == "e"
            || first_letter == "i"
            || first_letter == "o"
            || first_letter == "u"
        {
            updated_text.push_str(&format!("{}-hay ", word));
        } else {
            updated_text.push_str(&format!("{}-{}ay ", &word[1..], first_letter));
        }
    }

    updated_text
}
