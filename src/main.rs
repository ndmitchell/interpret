pub fn main() {
    for repeat in &[2, 3, 4] {
        let entries = interpret::entries(*repeat);

        let mut results = Vec::new();
        for e in entries {
            results.push(e.apply())
        }
        println!("{:?}", results);
        results.dedup();
        assert_eq!(results.len(), 1);
    }
}
