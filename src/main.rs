use std::ops::Add;

fn sum_numbers<T>(a: T, b: T) -> T
where T: Add<Output=T> + Copy + Clone{
  let result = a + b;
  result
}

fn main() {
    let sum = sum_numbers(2,5);
    println!("{}", sum);
}
