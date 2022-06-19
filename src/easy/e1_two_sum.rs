#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    #[test]
    fn test() {
        // 样式测试
        let res:Vec<i32> = two_sum(vec![1, 2, 3], 5);
        println!("{:?}", res);
        // 算法实现
        fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
            let mut hash: HashMap<i32, i32> = HashMap::new();
            for i in 0..nums.len() {
                let n = nums[i];
                if !hash.contains_key(&n) {
                    hash.insert(target - n, i as i32);
                } else {
                    match hash.get(&n) {
                        Some(&n) => {
                            return vec![i as i32, n];
                        }
                        None => {}
                    }
                }
            }
            vec![-1, -1]
        }
    }
}
