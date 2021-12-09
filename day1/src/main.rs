use std::fs;

fn main() {
    let file = fs::read_to_string("src/input.txt").expect("Error");
    let input: Vec<i32> = file
        .split("\n")
        .map(|line| match line.trim().parse() {
            Ok(line) => line,
            Err(_) => 0,
        })
        .collect();
    println!("{}", task1(&input));
    println!("{}", task2(&input));
}

fn task1(input: &Vec<i32>) -> i32 {
    input
        .iter()
        .zip(input.iter().skip(1))
        .map(|(a, b)| if a < b { 1 } else { 0 })
        .collect::<Vec<i32>>()
        .iter()
        .sum()
}

fn task2(input: &Vec<i32>) -> i32 {
    let window = input
        .iter()
        .zip(input.iter().skip(1))
        .zip(input.iter().skip(2))
        .map(|((a, b), c)| a + b + c)
        .collect::<Vec<i32>>();
    window
        .iter()
        .zip(window.iter().skip(1))
        .map(|(a, b)| if a < b { 1 } else { 0 })
        .collect::<Vec<i32>>()
        .iter()
        .sum()
}
