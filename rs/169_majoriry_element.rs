impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut count = 0;
        let mut num = nums[0];

        for val in nums{
            if count == 0{
                num = val;
            }

            if num == val{
                count +=1;
            }
            else{
                count -=1;
            }
        }

        num
    }
}