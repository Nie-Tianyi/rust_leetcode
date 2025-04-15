struct Solution;

impl Solution {
    #[allow(unused)]
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = Vec::new();

        // 如果长度不足三，直接返回空数组
        if nums.len() < 3 {
            return res;
        }

        let mut sorted_nums = nums;
        sorted_nums.sort();

        let len = sorted_nums.len();
        'outer: for x in 0..len - 2 {
            //跳过重复的第一个元素
            if x >= 1 && sorted_nums[x] == sorted_nums[x - 1] {
                continue 'outer;
            }

            let mut y = x + 1;
            let mut z = len - 1;
            'inner: while y < z {
                let sum = sorted_nums[x] + sorted_nums[y] + sorted_nums[z];
                match sum.cmp(&0) {
                    std::cmp::Ordering::Less => y += 1,
                    std::cmp::Ordering::Equal => {
                        res.push(vec![sorted_nums[x], sorted_nums[y], sorted_nums[z]]);
                        // 跳过重复的
                        while y < z && sorted_nums[y] == sorted_nums[y + 1] {
                            y += 1;
                        }
                        while y < z && sorted_nums[z] == sorted_nums[z - 1] {
                            z -= 1;
                        }
                        y += 1;
                        z -= 1;
                    }
                    std::cmp::Ordering::Greater => z -= 1,
                }
            }
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_1() {
        let ans = Solution::three_sum(vec![-1, 0, 1, 2, -1, -4]);
        assert_eq!(ans, vec![vec![-1, -1, 2], vec![-1, 0, 1]]);
    }
}
