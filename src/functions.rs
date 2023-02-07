pub fn run() {
    greetins("Hello", "Ronald");

    let add_sum = |n1: i32, n2: i32| n1 + n2;

    println!("C sum: {}", add_sum(3,4));

    let sum = add(5,5);
    println!("Sum: {}", sum);
}

fn greetins(greets: &str, name: &str) {
    println!("{} {}, nice to meet you.", greets, name);
}

fn add(n1: i32, n2: i32) -> i32{
    n1 + n2
}