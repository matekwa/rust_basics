pub fn run() {
    let vec1 = vec![1,2,3,4];
    let vec2 = &vec1;

    println!("Vec values: {:?}", (&vec1, vec2));
}