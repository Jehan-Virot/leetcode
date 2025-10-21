use std::collections::HashMap;


impl Solution {
    pub fn int_to_roman(num: i32) -> String {
        let map: HashMap<&str, i32> = [
            ("I", 1),
            ("IV", 4),
            ("V", 5),
            ("IX", 9),
            ("X", 10),
            ("XL", 40),
            ("L", 50),
            ("XC", 90),
            ("C", 100),
            ("CD", 400),
            ("D", 500),
            ("CM", 900),
            ("M", 1000),
        ].iter().cloned().collect();
        let order: Vec<&str> = vec!["M", "CM", "D", "CD", "C", "XC", "L", "XL", "X", "IX", "V", "IV", "I"];
        let mut output:String = String::new();
        let mut num = num;
        for keys in order.iter(){
            let value_for_key = *map.get(keys).unwrap() as i32;
            let mut tmp: i32 = num / value_for_key;
            if tmp > 0 {
                num = num % (tmp * value_for_key);
                for reach in 0..tmp {
                    output.push_str(keys);
                }
            }
        }
        output
    }
}