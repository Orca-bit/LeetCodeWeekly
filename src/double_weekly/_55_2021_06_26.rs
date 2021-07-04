use std::cmp::max;

struct Solution;

impl Solution {
    pub fn can_be_increasing(nums: Vec<i32>) -> bool {
        if nums.len() <= 2 {
            return true;
        }
        let mut p1 = 0;
        while p1 < nums.len() - 1 && nums[p1] < nums[p1 + 1] {
            p1 += 1;
        }
        if p1 == nums.len() - 1 {
            return true;
        }
        let mut p2 = nums.len() - 1;
        while p2 > 0 && nums[p2 - 1] < nums[p2] {
            p2 -= 1;
        }
        if p1 + 2 == p2 && nums[p1] < nums[p2] {
            return true;
        }
        if p1 + 1 == p2 {
            if p1 == 0 || (p1 > 0 && nums[p1 - 1] < nums[p2]) {
                return true;
            }
            if p2 == nums.len() - 1 || (p2 < nums.len() - 1 && nums[p1] < nums[p2 + 1]) {
                return true;
            }
        }
        false
    }

    pub fn remove_occurrences(s: String, part: String) -> String {
        let mut res = String::new();
        for c in s.chars() {
            res.push(c);
            if res.len() >= part.len() && res[res.len() - part.len()..] == part[..] {
                res.drain(res.len() - part.len()..);
            }
        }
        res
    }

    pub fn max_alternating_sum(nums: Vec<i32>) -> i64 {
        let mut odd = 0 as i64;
        let mut even = 0 as i64;
        for num in nums {
            odd = max(odd, even + num as i64);
            even = max(even, odd - num as i64);
        }
        odd
    }
}

#[test]
fn test1() {
    assert_eq!(Solution::can_be_increasing(vec![1, 2, 10, 5, 7]), true);
    assert_eq!(Solution::can_be_increasing(vec![2, 3, 1, 2]), false);
    assert_eq!(Solution::can_be_increasing(vec![1, 1, 1]), false);
}

#[test]
fn test2() {
    assert_eq!(Solution::remove_occurrences("daabcbaabcbc".to_string(), "abc".to_string()), "dab".to_string());
}

#[test]
fn test3() {
    assert_eq!(Solution::max_alternating_sum(vec![4, 2, 5, 3]), 7);
    assert_eq!(Solution::max_alternating_sum(vec![6, 2, 1, 2, 4, 5]), 10);
}

#[test]
fn test4() {

}
