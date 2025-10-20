use std::collections::HashMap;
impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let map: HashMap<char, i32> = [
            ('I', 1),
            ('V', 5),
            ('X', 10),
            ('L', 50),
            ('C', 100),
            ('D', 500),
            ('M', 1000),
        ]
        .into_iter()
        .collect();

        let mut number: i32 = 0;
        let mut previous: i32 = 0;
        for (index, chara) in s.chars().enumerate()  {
            let value = *map.get(&chara).unwrap();
            if index > 0 && value > previous {
                number += value - 2 * previous
            } else {
                number += value
            }
            previous = value;
        };
        number
    }
}