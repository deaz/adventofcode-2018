use std::collections::HashMap;
use std::collections::HashSet;

pub fn f(input: &str) -> String {
    let mut incoming = input
        .lines()
        .map(|line| {
            let re =
                regex::Regex::new(r"Step (.) must be finished before step (.) can begin.").unwrap();
            let caps = re.captures(line).unwrap();
            (caps.get(1).unwrap().as_str(), caps.get(2).unwrap().as_str())
        })
        .fold(HashMap::new(), |mut incoming, (from, to)| {
            incoming
                .entry(to)
                .or_insert_with(|| HashSet::new())
                .insert(from);
            incoming.entry(from).or_insert_with(|| HashSet::new());
            incoming
        });

    let mut result = String::new();
    while !incoming.is_empty() {
        let mut without_incoming = incoming
            .iter()
            .filter(|(_, vs)| vs.is_empty())
            .map(|(v, _)| v)
            .cloned()
            .collect::<Vec<_>>();
        without_incoming.sort();
        let next = without_incoming[0];
        result += next;
        incoming.remove(next);
        for vs in incoming.iter_mut().map(|v| v.1) {
            vs.remove(next);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin.";
        assert_eq!(f(input), "CABDFE");
    }
}
