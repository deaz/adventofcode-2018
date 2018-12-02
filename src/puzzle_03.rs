use std::collections::HashMap;

pub fn f(input: &str) -> String {
    input
        .lines()
        .map(|s| {
            let mut chars = HashMap::new();
            for c in s.chars() {
                let cur = chars.get(&c).unwrap_or(&0);
                chars.insert(c, cur + 1);
            }
            chars
                .values()
                .fold((false, false), |(double, triple), &count| {
                    if count == 2 {
                        (true, triple)
                    } else if count == 3 {
                        (double, true)
                    } else {
                        (double, triple)
                    }
                })
        })
        .fold([0, 0], |[double_count, triple_count], (double, triple)| {
            [double_count + double as u32, triple_count + triple as u32]
        })
        .iter()
        .product::<u32>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "abcdef\nbababc\nabbcde\nabcccd\naabcdd\nabcdee\nababab";
        assert_eq!(f(input), "12");
    }
}
