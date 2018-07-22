fn main() {
    let num = 9;
    let fibonacci = fib(num);

    println!("The {}th number in the fibonacci sequence is {}", num, fibonacci);
}

fn fib(number: i32) -> i32 {
    if number <= 1 {
        1
    } else {
        return fib(number - 1) + fib(number - 2);
    }
}

