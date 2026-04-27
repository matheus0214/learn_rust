fn main() {
    println!("90 F = {}C", fahrenheit_to_celsius(90.0));
    fibonacci(13);
}

fn fahrenheit_to_celsius(c: f32) -> f32 {
    (c - 32.0) * 5.0 / 9.0
}

fn fibonacci(n: u32) {
    let mut prev = 0;
    let mut next = 1;

    for _ in 0..n {
        print!("{prev} ");
        let tmp = prev;

        prev = next;
        next = tmp + next;
    }
    println!();
}
