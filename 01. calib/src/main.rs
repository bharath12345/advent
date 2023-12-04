use std::fs;

fn main() {
  let file_path = "./input.txt";//"./example.txt";
  let contents = fs::read_to_string(file_path)
    .expect("Should have been able to read the file");

  let lines = contents.split("\n");
  let mut numbers: Vec<u32> = Vec::new();
  for line in lines {
    let chars: Vec<char> = line.chars().collect();
    let nums: Vec<u32> = chars.iter().flat_map(|c| c.to_digit(10)).collect();
    let number: u32 = if nums.len() == 1 {
        let mut num = nums.first().unwrap().to_string();
        let num2 = num.clone();
        num.push_str(&num2);
        num.parse::<u32>().unwrap()
    } else if nums.len() < 1 {
       0
    } else {
       let mut first = nums.first().unwrap().to_string();
       let last = nums.last().unwrap().to_string();
       first.push_str(&last);
       first.parse::<u32>().unwrap()
    };
    println!("{}", number);
    numbers.push(number);
  }
  let sum: u32 = numbers.iter().sum();
  println!("sum = {}", sum);
}
