pub fn f(input: &str) -> String {
    input
        .lines()
        .map(|s| s.parse::<i32>().expect(&format!("hoho {}", s)))
        .sum::<i32>()
        .to_string()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let input1 = String::from("+1\n+1\n+1");
        assert_eq!(super::f(&input1), "3");
        let input2 = String::from("+1\n+1\n-2");
        assert_eq!(super::f(&input2), "0");
        let input3 = String::from("-1\n-2\n-3");
        assert_eq!(super::f(&input3), "-6");
    }
}
