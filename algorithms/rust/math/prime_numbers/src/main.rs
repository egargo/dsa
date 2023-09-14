use std::io;

fn prime(n: u128) -> Vec<u128> {
    let mut a: Vec<bool> = vec![true; (n + 1) as usize];

    for i in 2..=n {
        if a[i as usize] == true {
            for j in (i * i..=n).step_by(i as usize) {
                a[j as usize] = false;
            }
        }
    }

    a.iter()
        .enumerate()
        .filter(|(i, j)| *i >= 2 && *j == &true)
        .map(|(i, _)| i as u128)
        .collect::<Vec<u128>>()
}

fn main() {
    let mut range = String::new();

    io::stdin()
        .read_line(&mut range)
        .expect("Failed to read line");

    let range: u128 = range.trim().parse::<u128>().unwrap();

    println!("{:?}", prime(range));
}
