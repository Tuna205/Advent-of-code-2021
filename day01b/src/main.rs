fn main() {
    let input: Vec<usize> = include_str!("../input.txt")
        .lines()
        .map(|i| i.parse::<usize>().unwrap())
        .collect();

    let mut counter = 0;
    for i in 0..input.len() - 3 {
        if input.get(i).unwrap() < input.get(i + 3).unwrap() {
            counter += 1;
        }
    }

    println!("{}", counter);
}
