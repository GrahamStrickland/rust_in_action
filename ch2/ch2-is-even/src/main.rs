fn is_even(n: i32) -> bool {
    n % 2 == 0
}

fn main() {
    let n = 123456;
    let description = if is_even(n) {
        "even"
    } else {
        "odd"
    };
    println!("{} is {}", n, description);

    let n = 654321;
    let description = match is_even(n) {
        true => "even",
        false => "odd",
    };    
    println!("{} is {}", n, description);

    let n = loop {
        break 1234;
    };

    println!("{}", n);
}
