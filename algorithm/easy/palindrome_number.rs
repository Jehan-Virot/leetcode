impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        }
        if x == 0 {
            return true;
        }

        let num_digits = ((x as f64).log10().floor() as i32) + 1;

        for i in 0..(num_digits / 2) {
            let right_digit = (x / 10_i32.pow(i as u32)) % 10;
            let left_digit = (x / 10_i32.pow((num_digits - i - 1) as u32)) % 10;

            if left_digit != right_digit {
                return false;
            }
        }

        true
    }
}
