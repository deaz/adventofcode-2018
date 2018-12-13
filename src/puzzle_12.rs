pub fn f(input: &str) -> String {
    compute(input, 10000)
}

fn compute(input: &str, max_total_dist: i32) -> String {
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

    let mut region_area = 0;
    for i in field_left..field_right {
        for j in field_top..field_bottom {
            let mut total_dist = 0;
            for p in points.iter() {
                total_dist += dist((i, j), p);
            }
            if total_dist < max_total_dist {
                region_area += 1;
            }
        }
    }

    region_area.to_string()

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
        assert_eq!(compute(input, 32), "16");
    }
}
