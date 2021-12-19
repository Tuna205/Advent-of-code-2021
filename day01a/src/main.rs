fn main() {
    let mut input = include_str!("../input.txt")
        .lines()
        .map(|i| i.parse::<usize>().unwrap());

    let mut prev = input.next().unwrap();
    let mut counter = 0;
    for i in input {
        if prev < i{
            counter += 1;
        }
        prev = i
    }

    println!("{}", counter);
}
