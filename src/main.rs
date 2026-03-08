use std::collections::HashSet;

pub fn find_different_binary_string(nums: Vec<String>) -> String {
    let present : HashSet<u16> = nums.iter().map(|n| u16::from_str_radix(n, 2).unwrap()).collect();
    let max: usize = 2_usize.pow(nums.len() as u32);
    
    for n in (0..max).rev() {
        if !present.contains(&(n as u16)) {
            return format!("{:0width$b}", n, width = nums.len());
        }
    }

    "".to_string()
}

fn main() {
    let input = vec!["01".to_string(), "10".to_string()];
    let result = find_different_binary_string(input);
    println!("{}", result);
}