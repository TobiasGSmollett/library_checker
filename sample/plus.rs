use std::io::stdin;

fn main() {
    let mut input = String::new();
    let _result = stdin().read_line(&mut input);
    let mut nums = input.split_whitespace().map(|e| e.parse::<i32>().unwrap());
    let a = nums.next().unwrap();
    let b = nums.next().unwrap();
    println!("{}", a + b);
}
