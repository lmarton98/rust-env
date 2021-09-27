fn fibo(n: u32) -> u32 {
    if n <= 1 {
        1_u32
    } else {
        fibo(n - 1) + fibo(n - 2)
    }
}

fn main() {
    println!("Hello, fibo");
    for i in 1..12 {
        let fib = fibo(i);
        println!("N = {:?}, fibo(N) = {:?}", i, fib);
    }
}
