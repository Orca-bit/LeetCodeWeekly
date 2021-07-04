struct Solution;

impl Solution {
    pub fn build_array(nums: Vec<i32>) -> Vec<i32> {
        let mut res = vec![0; nums.len()];
        for i in 0..nums.len() {
            res[i] = nums[nums[i] as usize];
        }
        res
    }

    pub fn eliminate_maximum(dist: Vec<i32>, speed: Vec<i32>) -> i32 {
        let mut time = vec![0.; dist.len()];
        for i in 0..time.len() {
            time[i] = dist[i] as f64 / speed[i] as f64;
        }
        time.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
        if time[0] == 0. {
            return 0;
        }
        let mut res = 0;
        let mut past_time = 0;
        for i in 0..time.len() {
            if time[i] - (past_time as f64) <= 0. {
                break;
            }
            res += 1;
            past_time += 1;
        }
        res
    }

    pub fn count_good_numbers(n: i64) -> i32 {
        let odds = n >> 1;
        let evens = (n + 1) >> 1;
        let limit = 10_i64.pow(9) + 7;
        let pow = |a: i64, mut b: i64| -> i64 {
            let mut res = 1;
            let mut tmp = a;
            while b > 0 {
                if b & 1 != 0 {
                    res = res * tmp % limit;
                }
                tmp = tmp.pow(2) % limit;
                b >>= 1;
            }
            res
        };
        (pow(4, odds) * pow(5, evens) % limit) as i32
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::build_array(vec![5, 0, 1, 2, 3, 4]),
        vec![4, 5, 0, 1, 2, 3]
    );
    assert_eq!(Solution::eliminate_maximum(vec![1, 3, 4], vec![1, 1, 1]), 3);
    assert_eq!(
        Solution::eliminate_maximum(vec![1, 1, 2, 3], vec![1, 1, 1, 1]),
        1
    );
    assert_eq!(Solution::eliminate_maximum(vec![3, 2, 3], vec![2, 1, 1]), 3);
    assert_eq!(Solution::count_good_numbers(50), 564908303);
}
