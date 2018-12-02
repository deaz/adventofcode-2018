pub fn f(input: &str) -> String {
    let ids: Vec<_> = input.lines().collect();
    let mut res = String::new();

    'outer: for id1 in ids.iter() {
        for id2 in ids.iter() {
            res = id1
                .chars()
                .zip(id2.chars())
                .fold(String::new(), |acc, (c1, c2)| {
                    if c1 == c2 {
                        format!("{}{}", acc, c1)
                    } else {
                        acc
                    }
                });
            if res.chars().count() == id1.chars().count() - 1 {
                break 'outer;
            }
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "abcde\nfghij\nklmno\npqrst\nfguij\naxcye\nwvxyz";
        assert_eq!(f(input), "fgij");
    }
}
