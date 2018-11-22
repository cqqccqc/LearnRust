fn main() {
    for number in (1..4).rev() {
        println!("{}", number);
    }

    println!("cf {}", cf(100.0));

    println!("fib {}", fib(15));
}

fn cf(degree: f32) -> f32 {
    return degree * 9.0 / 5.0 + 32.0;
}

fn fib(n: i32)-> i32 {
    if n < 1 {
        panic!("n < 1")
    }
    if n == 1 || n == 2 {
        return 1;
    }
    
    return fib(n - 2) + fib(n - 1);
}
