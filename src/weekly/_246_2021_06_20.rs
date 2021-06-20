use std::cmp::min;

fn largest_odd_number(num: String) -> String {
    let v: Vec<_> = num
        .trim()
        .chars()
        .rev()
        .map(|c| c.to_string().parse::<u32>().unwrap())
        .collect();
    let mut index = -1;
    for (i, &item) in v.iter().enumerate() {
        if item % 2 != 0 {
            index = i as i32;
            break;
        }
    }
    if index == -1 {
        "".to_string()
    } else {
        v.iter()
            .skip(index as usize)
            .rev()
            .map(|&i| i.to_string())
            .collect::<String>()
    }
}

fn number_of_rounds(start_time: String, finish_time: String) -> i32 {
    let mut overnight = false;
    if start_time > finish_time {
        overnight = true;
    }
    let start_time: Vec<_> = start_time
        .split(':')
        .map(|c| c.parse::<u32>().unwrap())
        .collect();
    let mut finish_time: Vec<_> = finish_time
        .split(':')
        .map(|c| c.parse::<u32>().unwrap())
        .collect();
    if overnight {
        finish_time[0] += 24;
    }
    let res = (finish_time[0] + 1 - start_time[0]) * 4
        - (start_time[1] + 14) / 15
        - (59 - finish_time[1]) / 15
        - 1;

    res as i32
}

fn count_sub_islands(grid1: Vec<Vec<i32>>, mut grid2: Vec<Vec<i32>>) -> i32 {
    let rows = grid2.len();
    if rows == 0 {
        return 0;
    }
    let cols = grid2[0].len();
    let mut res = 0;
    for i in 0..rows {
        for j in 0..cols {
            if grid2[i][j] == 1 {
                let mut is_sub = true;
                process(&grid1, &mut grid2, &mut is_sub, i as i32, j as i32);
                if is_sub {
                    res += 1;
                }
            }
        }
    }
    res
}

fn process(
    grid1: &Vec<Vec<i32>>,
    grid2: &mut Vec<Vec<i32>>,
    is_sub: &mut bool,
    row: i32,
    col: i32,
) {
    if row < 0
        || row >= grid2.len() as i32
        || col < 0
        || col >= grid2[0].len() as i32
        || grid2[row as usize][col as usize] != 1
    {
        return;
    }
    if grid1[row as usize][col as usize] != 1 {
        *is_sub = false;
    }
    grid2[row as usize][col as usize] = -1;
    let neighbors = vec![(-1, 0), (0, -1), (1, 0), (0, 1)];
    for (i, j) in neighbors {
        process(grid1, grid2, is_sub, row + i, col + j);
    }
}

fn find_min_difference(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
    let mut pre = vec![vec![0; 101]; nums.len() + 1];
    for (i, &num) in nums.iter().enumerate() {
        pre[i + 1] = pre[i].clone();
        pre[i + 1][num as usize] += 1;
    }
    let mut res = vec![];
    for q in queries {
        let mut table = vec![];
        for i in 1..=100 {
            if pre[q[1] as usize + 1][i] - pre[q[0] as usize][i] > 0 {
                table.push(i as i32);
            }
        }
        if table.len() == 1 {
            res.push(-1);
        } else {
            let mut tem = i32::MAX;
            for i in 1..table.len() {
                tem = min(tem, (table[i] - table[i - 1]).abs());
            }
            res.push(tem);
        }
    }
    res
}

#[test]
fn test3() {
    let nums = vec![1, 3, 4, 8];
    let q = vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![0, 3]];
    assert_eq!(find_min_difference(nums, q), vec![2, 1, 4, 1]);
}

#[test]
fn test2() {
    let grid1 = vec![
        vec![1, 0, 1, 0, 1],
        vec![1, 1, 1, 1, 1],
        vec![0, 0, 0, 0, 0],
        vec![1, 1, 1, 1, 1],
        vec![1, 0, 1, 0, 1],
    ];
    let grid2 = vec![
        vec![0, 0, 0, 0, 0],
        vec![1, 1, 1, 1, 1],
        vec![0, 1, 0, 1, 0],
        vec![0, 1, 0, 1, 0],
        vec![1, 0, 0, 0, 1],
    ];
    assert_eq!(count_sub_islands(grid1, grid2), 2);
}

#[test]
fn test1() {
    assert_eq!(
        number_of_rounds("12:01".to_string(), "12:44".to_string()),
        1
    );
    assert_eq!(
        number_of_rounds("20:00".to_string(), "06:00".to_string()),
        40
    );
    assert_eq!(
        number_of_rounds("00:00".to_string(), "23:59".to_string()),
        95
    );
    assert_eq!(
        number_of_rounds("00:01".to_string(), "00:00".to_string()),
        95
    );
}

#[test]
fn test() {
    assert_eq!(largest_odd_number("35427".to_string()), "35427".to_string());
}