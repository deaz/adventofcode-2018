use std::collections::HashSet;

#[derive(Debug)]
struct State {
    seen_items: HashSet<i32>,
    current: i32,
}

pub fn f(input: &str) -> String {
    input
        .lines()
        .map(|s| s.parse::<i32>().unwrap())
        .cycle()
        .scan(
            State {
                seen_items: HashSet::new(),
                current: 0,
            },
            |state, item| {
                if state.seen_items.contains(&state.current) {
                    None
                } else {
                    state.seen_items.insert(state.current);
                    state.current += item;
                    Some(state.current)
                }
            },
        )
        .last()
        .unwrap()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    tests! {
        test_1: ("+1\n-1", "0"),
        test_2: ("+3\n+3\n+4\n-2\n-4", "10"),
        test_3: ("+7\n+7\n-2\n-7\n-4", "14"),
    }
}
