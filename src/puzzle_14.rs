use std::collections::HashMap;
use std::collections::HashSet;

pub fn f(input: &str) -> String {
    compute(input, 5, 60)
}

fn compute(input: &str, workers_num: usize, duration: u32) -> String {
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

    let mut time = 0;
    let mut workers = vec![(0, ""); workers_num];
    while !incoming.is_empty() || workers.iter().any(|&(left, _)| left != 0){
        time += 1;

        for (ref mut left, job) in workers.iter_mut() {
            if *left == 0 {
                continue
            }

            *left -= 1;
            if *left == 0 {
                for vs in incoming.iter_mut().map(|v| v.1) {
                    vs.remove(job);
                }
            }
        }

        if workers.iter().all(|&(left, _)| left > 0) {
            continue;
        }

        let mut without_incoming = incoming
            .iter()
            .filter(|(_, vs)| vs.is_empty())
            .map(|(v, _)| v)
            .cloned()
            .collect::<Vec<_>>();
        without_incoming.sort();

        let free_workers = workers.iter_mut().filter(|(left, _)| left == &0);
        for (w, job) in free_workers.zip(without_incoming) {
            w.0 = duration + (job.chars().next().unwrap() as u32 - 'A' as u32 + 1);
            w.1 = job;
            incoming.remove(job);
        }
    }

    (time - 1).to_string()
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
        assert_eq!(compute(input, 2, 0), "15");
    }
}
