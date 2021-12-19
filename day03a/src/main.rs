fn main() {
    let mut sum: Vec<u32> = vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

    include_str!("../input.txt")
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .for_each(|v: Vec<u32>| sum = sum_vec(&sum, v));

    let gamma: Vec<u32> = sum.iter().map(|i| if i > &500 { 1 } else { 0 }).collect();
    let epsilon: Vec<u32> = sum.iter().map(|i| if i > &500 { 0 } else { 1 }).collect();

    println!("{:?}", gamma);
    println!("{:?}", to_decimal_num(&gamma) * to_decimal_num(&epsilon));
}

fn sum_vec(a: &Vec<u32>, b: Vec<u32>) -> Vec<u32> {
    return a.iter().zip(b.iter()).map(|(i, j)| i + j).collect();
}

fn to_decimal_num(vec: &Vec<u32>) -> isize {
    let binary_string: String = vec
        .iter()
        .map(|i| char::from_digit(*i, 10).unwrap())
        .collect();
    return isize::from_str_radix(&binary_string, 2).unwrap();
}
