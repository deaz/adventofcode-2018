use chrono::NaiveDateTime;
use chrono::Timelike;
use std::collections::HashMap;

enum Type {
    FallAsleep,
    WakeUp,
    BeginShift(u32),
}

pub fn f(input: &str) -> String {
    let mut schedule = input
        .lines()
        .map(|s| {
            let re = regex::Regex::new(r"\[(?P<date>.*)\] (?P<action>Guard #(?P<guard>\d+)|.*)")
                .unwrap();
            let caps = re.captures(s).unwrap();
            let r#type = match (caps.name("guard"), caps.name("action")) {
                (Some(id), _) => Type::BeginShift(id.as_str().parse::<u32>().unwrap()),
                (None, Some(action)) if action.as_str() == "falls asleep" => Type::FallAsleep,
                (None, Some(_)) => Type::WakeUp,
                _ => panic!("Parsing error"),
            };
            let time = NaiveDateTime::parse_from_str(
                caps.name("date").unwrap().as_str(),
                "%Y-%m-%d %H:%M",
            )
            .unwrap();
            (time, r#type)
        })
        .collect::<Vec<_>>();
    schedule.sort_by(|(a, _), (b, _)| a.cmp(b));

    let mut state = HashMap::new();
    let mut cur_id = 0;
    let mut cur_start = 0;
    for (time, r#type) in schedule {
        match r#type {
            Type::BeginShift(id) => cur_id = id,
            Type::FallAsleep => cur_start = time.minute(),
            _ => {
                state
                    .entry(cur_id)
                    .or_insert_with(|| Vec::new())
                    .push((cur_start, time.minute()));
            }
        };
    }

    let id = state
        .iter()
        .map(|(id, intervals)| (id, intervals.iter().fold(0, |acc, (a, b)| acc + (b - a))))
        .max_by_key(|(_, duration)| *duration)
        .unwrap()
        .0;

    let minute = *state
        .get(id)
        .unwrap()
        .iter()
        .fold(HashMap::new(), |mut freqs, (a, b)| {
            for i in *a..*b {
                freqs.entry(i).and_modify(|m| *m += 1).or_insert(1);
            }
            freqs
        })
        .iter()
        .max_by_key(|(_, count)| *count)
        .unwrap()
        .0;

    (id * minute).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "\
        [1518-11-01 00:00] Guard #10 begins shift
[1518-11-01 00:05] falls asleep
[1518-11-01 00:25] wakes up
[1518-11-01 00:30] falls asleep
[1518-11-01 00:55] wakes up
[1518-11-01 23:58] Guard #99 begins shift
[1518-11-02 00:40] falls asleep
[1518-11-02 00:50] wakes up
[1518-11-03 00:05] Guard #10 begins shift
[1518-11-03 00:24] falls asleep
[1518-11-03 00:29] wakes up
[1518-11-04 00:02] Guard #99 begins shift
[1518-11-04 00:36] falls asleep
[1518-11-04 00:46] wakes up
[1518-11-05 00:03] Guard #99 begins shift
[1518-11-05 00:45] falls asleep
[1518-11-05 00:55] wakes up";
        assert_eq!(f(input), "240");
    }
}
