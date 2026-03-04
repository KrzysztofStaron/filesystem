impl Solution {
    pub fn num_special(mat: Vec<Vec<i32>>) -> i32 {
        let mut ans = 0;

        for i in 0..mat.len() {
            for j in 0..mat[i].len() {
                if Self::is_pos_special(&mat, i, j) {
                    ans += 1;
                }
            }
        }

        return ans;
    }

    fn is_pos_special(mat: &Vec<Vec<i32>>, y: usize, x: usize) -> bool {
        if mat[y][x] == 0 {
            return false;
        }


        if mat[y].iter().filter(|&&x| x == 1).count() != 1 {
            return false;
        }

        if mat.iter().filter(|row| row[x] == 1).count() != 1 {
            return false;
        }

        return true;
    }
}