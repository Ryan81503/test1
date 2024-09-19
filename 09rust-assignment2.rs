fn is_even(n: i32) -> bool {
    n % 2 == 0
}

fn main() {
    let numbers = [12, 15, 9, 20, 33, 50, 22, 30, 18, 6];

    // Analyze each number
    for &num in numbers.iter() {
        if is_even(num) {
            println!("{} is even", num);
        } else {
            println!("{} is odd", num);
        }

        if num % 3 == 0 && num % 5 == 0 {
            println!("FizzBuzz");
        } else if num % 3 == 0 {
            println!("Fizz");
        } else if num % 5 == 0 {
            println!("Buzz");
        }
    }

    // Sum of all numbers using a while loop
    let mut sum = 0;
    let mut i = 0;
    while i < numbers.len() {
        sum += numbers[i];
        i += 1;
    }
    println!("Sum of all numbers: {}", sum);

    // Finding the largest number
    let mut largest = numbers[0];
    for &num in numbers.iter() {
        if num > largest {
            largest = num;
        }
    }
    println!("The largest number is: {}", largest);
}