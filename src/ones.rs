impl Solution {
    fn check_ones_segment(s: String) -> bool {
        let mut current_char: char = s.chars().nth(0).unwrap();
        let mut clusters: u8 = 1;

        for c in s.chars() {
            if c != current_char {
                clusters += 1;
                current_char = c;
            }

            if clusters >= 3 {
                return false;
            }
        }
        
        return true;
    }
}