use std::ops::Add;
use std::time::{Duration, Instant};

fn sum_numbers<T>(a: T, b: T) -> T
where T: Add<Output=T> + Copy + Clone{
  let result = a + b;
  result
}

fn findpi() -> f64 {
    let mut sub_results = vec![0.0f64; 100_000_000];

    let mut divisor = 1.0;
    let mut result: f64;

    for i in 0..100_000_000  {
        sub_results[i] = 4.0 / divisor;
        divisor = divisor + 2.0;
    }
    result = 0.0;

    for i in 0..100_000_000  {
        if i % 2 == 0 {
            result = result + sub_results[i];
        }
        else {
            result = result - sub_results[i];
        }
    }
	
	result
}

fn main() {
    let now = Instant::now();
    let sum = sum_numbers(2,5);
    println!("{}", sum);
	
	let pi = findpi();
	let new_now = Instant::now();
	println!("{}", pi);
	println!("{:?}", new_now.saturating_duration_since(now));
}
