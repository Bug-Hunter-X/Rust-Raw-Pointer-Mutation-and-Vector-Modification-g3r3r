fn main() {
    let mut v = vec![1, 2, 3];
    let index_to_modify = 0;
    v[index_to_modify] = 4; // Safe and idiomatic way to modify elements
    println!("Modified Vector: {:?}", v);
} 