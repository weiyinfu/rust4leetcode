struct Solution;

impl Solution {
    pub fn can_complete_circuit(mut gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        for start in 0..gas.len() {
            let mut now = gas[start];
            let mut can = true;
            for i in 0..gas.len() {
                now -= cost[(start + i) % gas.len()];
                if now < 0 {
                    can = false;
                    break;
                }
                now += gas[(start + i + 1) % gas.len()];
            }
            if can {
                return start as i32;
            }
        }
        -1
    }
}

fn main() {
    let gas = vec![1, 2, 3, 4, 5];
    let cost = vec![3, 4, 5, 1, 2];
    let ans = Solution::can_complete_circuit(gas, cost);
    println!("{}", ans);
}