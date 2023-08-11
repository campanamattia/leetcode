impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut count = 0;
        let mut i = 0;

        while i < nums.len() {
            let mut k = i + 1;
            while k < nums.len() && nums[k] == nums[i] {
                k += 1;
            }

            if k == i + 1 {
                nums[count] = nums[i];
                count += 1;
            } else {
                nums[count] = nums[i];
                count += 1;
                nums[count] = nums[i];
                count += 1;
            }

            i = k;
        }

        count as i32
    }
}