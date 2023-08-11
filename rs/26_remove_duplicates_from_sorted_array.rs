impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut count = 1;

        for index in 1..nums.len(){
            if nums[index] > nums[index - 1]{
                nums[count] = nums[index];
                count +=1;
            }
        }

        count as i32
    }
}