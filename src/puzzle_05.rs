use std::collections::HashMap;

pub fn f(input: &str) -> String {
    input
        .lines()
        .map(|s| -> (u32, u32, u32, u32) {
            let re = regex::Regex::new(r"@ (\d+),(\d+): (\d+)x(\d+)").unwrap();
            let cap = re.captures_iter(s).next().unwrap();
            (
                cap[1].parse().unwrap(),
                cap[2].parse().unwrap(),
                cap[3].parse().unwrap(),
                cap[4].parse().unwrap(),
            )
        })
        .fold(HashMap::new(), |mut map, (x, y, a, b)| {
            for i in x..(x + a) {
                for j in y..(y + b) {
                    let prev = map.get(&(i, j)).unwrap_or(&0);
                    map.insert((i, j), prev + 1);
                }
            }
            map
        })
        .values()
        .fold(0, |acc, &x| acc + if x > 1 { 1 } else { 0 })
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "#1 @ 1,3: 4x4\n#2 @ 3,1: 4x4\n#3 @ 5,5: 2x2";
        assert_eq!(f(input), "4");
    }
}
