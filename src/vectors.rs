pub fn run() {
    let mut numbers: Vec<i32> = vec![1,2,3,4,5];

    for x in numbers.iter_mut(){
        *x *=2;
    }
    print!("Vector values {:?} ", numbers);
}