use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn max_product_difference(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();
        let n = nums.len();
        nums[n - 1] * nums[n - 2] - (nums[0] * nums[1])
    }

    pub fn wonderful_substrings(word: String) -> i64 {
        let mut map = HashMap::new();
        map.insert(0, 1);
        let mut mask = 0;
        let mut res = 0;
        for ch in word.bytes() {
            let index = (ch - b'a') as i32;
            mask ^= 1 << index;
            if let Some(&value) = map.get(&mask) {
                res += value;
            }
            for i in 0..10 {
                let mask_pre = mask ^ (1 << i);
                if let Some(&value) = map.get(&mask_pre) {
                    res += value;
                }
            }
            *map.entry(mask).or_default() += 1;
        }
        res
    }

    pub fn wonderful_substrings_1(word: String) -> i64 {
        let mut map = vec![0; 1 << 10];
        map[0] = 1;
        let mut res = 0;
        let mut mask = 0;
        for ch in word.bytes() {
            mask ^= 1 << (ch - b'a');
            res += map[mask];
            for i in 0..10 {
                res += map[mask ^ (1 << i)];
            }
            map[mask] += 1;
        }
        res
    }

    pub fn ways_to_build_rooms(prev_rooms: Vec<i32>) -> i32 {

        0
    }
}

#[test]
fn test() {
    assert_eq!(Solution::max_product_difference(vec![5, 6, 2, 7, 4]), 34);
}

#[test]
fn test2() {
    assert_eq!(Solution::wonderful_substrings("aba".to_string()), 4);
    assert_eq!(Solution::wonderful_substrings_1("aba".to_string()), 4);
}
