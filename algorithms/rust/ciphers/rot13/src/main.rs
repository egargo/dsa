use std::io;

fn rot13(message: &str) -> String {
    let mut rot = String::new();

    for c in message.chars() {
        if c.is_alphabetic() {
            let check = c as u8 + 13;

            if c.is_uppercase() {
                if check > 90 {
                    rot.push((check - 26) as char);
                } else {
                    rot.push(check as char);
                }
            } else {
                if check > 122 {
                    rot.push((check - 26) as char);
                } else {
                    rot.push(check as char);
                }
            }
        } else {
            rot.push(c);
        }
    }

    rot.to_string()
}

fn main() {
    let mut message = String::new();

    io::stdin()
        .read_line(&mut message)
        .expect("Failed to read line");

    println!("{}", rot13(&message));
}

#[test]
fn test() {
    assert_eq!(
        rot13("The quick brown fox jumps over the lazy dog."),
        "Gur dhvpx oebja sbk whzcf bire gur ynml qbt."
    );

    assert_eq!(
        rot13("Lorem ipsum dolor sit amet, qui minim labore adipisicing minim sint cillum sint consectetur cupidatat."),
        "Yberz vcfhz qbybe fvg nzrg, dhv zvavz ynober nqvcvfvpvat zvavz fvag pvyyhz fvag pbafrpgrghe phcvqngng."
    )
}
