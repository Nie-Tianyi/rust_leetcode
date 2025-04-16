struct Solution;

impl Solution {
    #[allow(unused)]
    pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        // 如果长度不足4，则直接返回空数组
        if nums.len() < 4 {
            return Vec::new();
        }

        // 深度优先查找
        let mut nums = nums;
        nums.sort(); // 先排序
        let mut res = Vec::new();

        // 深度优先算法
        'l1: for i1 in 0..nums.len() - 3 {
            if i1 != 0 && nums[i1] == nums[i1 - 1] {
                // 跳过重复的元素
                continue 'l1;
            }
            'l2: for i2 in i1 + 1..nums.len() - 2 {
                if i2 != i1 + 1 && nums[i2] == nums[i2 - 1] {
                    // 跳过重复的元素
                    continue 'l2;
                }
                'l3: for i3 in i2 + 1..nums.len() - 1 {
                    if i3 != i2 + 1 && nums[i3] == nums[i3 - 1] {
                        // 跳过重复元素
                        continue 'l3;
                    }
                    'l4: for i4 in i3 + 1..nums.len() {
                        if i4 != i3 + 1 && nums[i4] == nums[i4 - 1] {
                            // 跳过重复元素
                            continue 'l4;
                        }

                        let num1 = nums[i1] as i64;
                        let num2 = nums[i2] as i64;
                        let num3 = nums[i3] as i64;
                        let num4 = nums[i4] as i64;

                        if num1 + num2 + num3 + num4 == target as i64 {
                            res.push(vec![nums[i1], nums[i2], nums[i3], nums[i4]])
                        }
                    }
                }
            }
        }

        res
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_1() {}
}
