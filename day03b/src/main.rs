//TODO defineatly can be more efficiant

fn main() {
    let pom: Vec<u32> = vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let mut occur: Vec<u32> = pom.clone();

    let mut input: Vec<Vec<u32>> = include_str!("../input.txt")
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    input
        .iter()
        .for_each(|v: &Vec<u32>| occur = sum_vec(&occur, v));

    let mut oxygen: Vec<u32> = vec![];

    for i in 0..occur.len() {
        if occur[i] < (input.len() / 2) as u32 {
            input = input.into_iter().filter(|v| v[i] == 1).collect();
            occur = pom.clone();
            input
                .iter()
                .for_each(|v: &Vec<u32>| occur = sum_vec(&occur, v));
        } else {
            input = input.into_iter().filter(|v| v[i] == 0).collect();
            occur = pom.clone();
            input
                .iter()
                .for_each(|v: &Vec<u32>| occur = sum_vec(&occur, v));
        }
        //ako je zadnji
        if input.len() == 1 {
            oxygen = input.first().unwrap().to_vec();
            break;
        }
    }
    println!("{:?}", oxygen);
    let oxygen = to_decimal_num(&oxygen) as u32;

    let mut input: Vec<Vec<u32>> = include_str!("../input.txt")
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    input
        .iter()
        .for_each(|v: &Vec<u32>| occur = sum_vec(&occur, v));

    let mut co2: Vec<u32> = vec![];

    for i in 0..occur.len() {
        if occur[i] >= (input.len() / 2) as u32 {
            input = input.into_iter().filter(|v| v[i] == 1).collect();
            occur = pom.clone();
            input
                .iter()
                .for_each(|v: &Vec<u32>| occur = sum_vec(&occur, v));
        } else {
            input = input.into_iter().filter(|v| v[i] == 0).collect();
            occur = pom.clone();
            input
                .iter()
                .for_each(|v: &Vec<u32>| occur = sum_vec(&occur, v));
        }
        //ako je zadnji
        if input.len() == 1 {
            co2 = input.first().unwrap().to_vec();
            break;
        }
    }
    println!("{:?}", co2);
    let co2 = to_decimal_num(&co2) as u32;

    println!("{}", oxygen * co2);
}

fn sum_vec(a: &Vec<u32>, b: &Vec<u32>) -> Vec<u32> {
    return a.iter().zip(b.iter()).map(|(i, j)| i + j).collect();
}

fn to_decimal_num(vec: &Vec<u32>) -> isize {
    let binary_string: String = vec
        .iter()
        .map(|i| char::from_digit(*i, 10).unwrap())
        .collect();
    return isize::from_str_radix(&binary_string, 2).unwrap();
}
