pub fn f(input: &str) -> String {
    ('a' as u32..='z' as u32)
        .map(|code| -> String {
            input
                .clone()
                .chars()
                .filter(|ch| ch.to_ascii_lowercase() as u32 != code)
                .collect()
        })
        .map(react)
        .min()
        .unwrap()
        .to_string()
}

fn react(s: String) -> usize {
    let mut tmp: Vec<_> = s.trim().chars().collect();
    let mut out;
    loop {
        out = Vec::new();
        let mut changed = false;
        let mut i = 0;
        while i + 1 < tmp.len() {
            if tmp[i] == tmp[i + 1]
                || tmp[i].to_ascii_lowercase() != tmp[i + 1].to_ascii_lowercase()
            {
                out.push(tmp[i]);
            } else {
                changed = true;
                i += 1;
            }
            i += 1;
        }
        if i + 1 == tmp.len() {
            out.push(tmp[i]);
        }
        if !changed {
            break;
        }
        tmp = out;
    }
    out.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    tests! {
        test_1: ("dabAcCaCBAcCcaDA", "4"),
    }
}
