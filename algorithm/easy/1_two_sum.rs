use std::collections::HashMap;
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut solution: Vec<i32>  = Vec::new();
        let mut verif: HashMap<&i32, i32> = HashMap::new();
        for (index, number) in nums.iter().enumerate(){
            let tmp:i32 = target - number;
            if verif.contains_key(&tmp){
                solution.push(verif[&tmp]);
                solution.push(index as i32);
                return solution;
            };
            verif.insert(number, index as i32);
        };
        solution
    }
}