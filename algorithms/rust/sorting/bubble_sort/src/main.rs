fn sort(numbers: Vec<u32>) -> Vec<u32> {
    let mut numbers = numbers;

    for i in 0..numbers.len() {
        for j in 0..numbers.len() - (i + 1) {
            let left = numbers[j];
            let right = numbers[j + 1];

            if left > right {
                numbers[j] = right;
                numbers[j + 1] = left;
            }
        }
    }

    numbers
}

fn main() {
    println!("{:?}", sort(vec![5, 4, 3, 2, 1, 10, 9, 8, 7, 6]));
    println!("{:?}", sort(vec![7, 7, 34, 99, 57, 19, 8, 74, 10, 17]));
    println!("{:?}", sort(vec![10, 0, 9, 10, 4, 2, 5, 7, 10, 9]));
    println!("{:?}", sort(vec![45, 62, 34, 82, 34, 20, 96, 48, 89, 88]));
    println!(
        "{:?}",
        sort(vec![273, 738, 212, 677, 161, 502, 616, 82, 937, 900])
    );
}
