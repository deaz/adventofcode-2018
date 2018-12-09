pub fn f(input: &str) -> String {
    let mut tmp: Vec<_> = input.trim().chars().collect();
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
    (out.len()).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    tests! {
        test_1: ("dabAcCaCBAcCcaDA", "10"),
        test_2: ("aA", "0"),
        test_3: ("abAB", "4"),
        test_4: ("aabAAB", "6"),
    }
}
