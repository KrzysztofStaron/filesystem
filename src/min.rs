fn main() {
    println!("{}", min_operations("0100".to_string()));
    println!("{}", min_operations("10".to_string()));
    println!("{}", min_operations("1111".to_string()));
}

// s1: 0101
// s2: 1010

fn min_operations(s: String) -> i32 {
    let mut changes_s1 = 0;
    let mut changes_s2 = 0;

    for (index, c) in s.chars().enumerate() {
        let expected_s1 = if index % 2 == 0 { '0' } else { '1' };
        let expected_s2 = if index % 2 == 0 { '1' } else { '0' };

        if c != expected_s1 {
            changes_s1 += 1;
        }

        if c != expected_s2 {
            changes_s2 += 1;
        }

    }

    return changes_s1.min(changes_s2);
}