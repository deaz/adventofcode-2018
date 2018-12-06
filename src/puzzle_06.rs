use std::collections::HashMap;
use std::collections::HashSet;

pub fn f(input: &str) -> String {
    input
        .lines()
        .map(|s| -> (u32, u32, u32, u32, u32) {
            let re = regex::Regex::new(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();
            let cap = re.captures_iter(s).next().unwrap();
            (
                cap[1].parse().unwrap(),
                cap[2].parse().unwrap(),
                cap[3].parse().unwrap(),
                cap[4].parse().unwrap(),
                cap[5].parse().unwrap(),
            )
        })
        .fold(
            (HashSet::new(), HashMap::new()),
            |(mut set, mut map), (id, x, y, a, b)| {
                let mut overlap = false;
                for i in x..(x + a) {
                    for j in y..(y + b) {
                        let &prev_id = map.get(&(i, j)).unwrap_or(&id);
                        if prev_id != id {
                            set.remove(&prev_id);
                            overlap = true;
                        }
                        map.insert((i, j), id);
                    }
                }
                if !overlap {
                    set.insert(id);
                }
                (set, map)
            },
        )
        .0
        .iter()
        .next()
        .unwrap()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "#1 @ 1,3: 4x4\n#2 @ 3,1: 4x4\n#3 @ 5,5: 2x2";
        assert_eq!(f(input), "3");
    }
}
