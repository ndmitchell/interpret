pub fn main() {
    let entries = interpret::entries(2);

    let mut results = Vec::new();
    for e in entries {
        results.push(e.apply())
    }
    println!("{:?}", results);
    results.dedup();
    assert_eq!(results.len(), 1);
}
