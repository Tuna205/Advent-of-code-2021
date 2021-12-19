use regex::Regex;

fn main() {
    let re = Regex::new(r#"^(.+) (\d+)$"#).unwrap();

    let mut horizontal = 0;
    let mut vertical = 0;
    let mut aim = 0;

    include_str!("../input.txt")
        .lines()
        .map(|l| re.captures(l).unwrap())
        .for_each(|l| {
            println!("{:?}", l);
            if l[1] == *"up" {
                aim -= l[2].parse::<i32>().unwrap();
            } else if l[1] == *"down" {
                aim += l[2].parse::<i32>().unwrap();
            } else if l[1] == *"forward" {
                let X = l[2].parse::<i32>().unwrap();
                horizontal += X;
                vertical += aim * X;
            }
        });

    println!("{}", horizontal);

    println!("{}", vertical);

    println!("{}", horizontal * vertical);
}
