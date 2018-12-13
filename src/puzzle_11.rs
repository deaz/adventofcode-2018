use std::collections::HashMap;
use std::collections::HashSet;

pub fn f(input: &str) -> String {
    let points: Vec<_> = input
        .lines()
        .map(|line| {
            let re = regex::Regex::new(r"(\d+), (\d+)").unwrap();
            let caps = re.captures(line).unwrap();
            (
                caps.get(1).unwrap().as_str().parse::<i32>().unwrap(),
                caps.get(2).unwrap().as_str().parse::<i32>().unwrap(),
            )
        })
        .collect();
    let (top, bottom, left, right) = points.iter().fold(
        (points[0].1, points[0].1, points[0].0, points[0].0),
        |(top, bottom, left, right), &(x, y)| {
            let new_top = if y > top { top } else { y };
            let new_bottom = if y < bottom { bottom } else { y };
            let new_left = if x > left { left } else { x };
            let new_right = if x < right { right } else { x };
            (new_top, new_bottom, new_left, new_right)
        },
    );

    // make field with borders of double initial field width
    let field_top = top - 2 * (bottom - top);
    let field_bottom = bottom + 2 * (bottom - top);
    let field_left = left - 2 * (right - left);
    let field_right = right + 2 * (right - left);

    let mut excluded = HashSet::new();
    let mut areas = HashMap::new();
    for i in field_left..field_right {
        for j in field_top..field_bottom {
            let mut min_dist = None;
            let mut min_dist_point = None;
            let mut has_equal_distances = false;
            for p in points.iter() {
                let d = dist((i, j), p);
                match min_dist {
                    None => {
                        min_dist = Some(d);
                        min_dist_point = Some(p)
                    }
                    Some(x) if x > d => {
                        min_dist = Some(d);
                        min_dist_point = Some(p);
                        has_equal_distances = false;
                    }
                    Some(x) if x == d => has_equal_distances = true,
                    _ => continue,
                };
            }
            if has_equal_distances {
                continue;
            }
            *areas.entry(min_dist_point.unwrap()).or_insert(0) += 1;
            if i == field_left || i + 1 == field_right || j + 1 == field_bottom || j == field_top {
                excluded.insert(min_dist_point.unwrap());
            }
        }
    }

    areas
        .iter()
        .filter(|&(p, _d)| !excluded.contains(p))
        .max_by_key(|&(_p, d)| d)
        .unwrap()
        .1
        .to_string()
}

fn dist((x1, y1): (i32, i32), (x2, y2): &(i32, i32)) -> i32 {
    (x1 - x2).abs() + (y1 - y2).abs()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "1, 1\n\
                     1, 6\n\
                     8, 3\n\
                     3, 4\n\
                     5, 5\n\
                     8, 9";
        assert_eq!(f(input), "17");
    }
}
