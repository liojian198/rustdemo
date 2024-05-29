#[cfg(test)]
mod test {
    use std::collections::HashMap;

    #[test]
    fn hashmap_demo() {
        let text = "hello world wonderful world";
        let mut map = HashMap::new();
        for word in text.split_whitespace() {
            let count = map.entry(word).or_insert(0);
            *count += 1;
        }
        println!("{:?}", map);
    }
}