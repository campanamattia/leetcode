impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut i = m as usize - 1;
        let mut j = n as usize - 1;
        let mut k = (m + n) as usize - 1;

        while i < nums1.len() && j < nums2.len() {
            if nums1[i] > nums2[j] {
                nums1[k] = nums1[i];
                i = i.wrapping_sub(1);
            } else {
                nums1[k] = nums2[j];
                j = j.wrapping_sub(1);
            }
            k = k.wrapping_sub(1);
        }

        while j < nums2.len() {
            nums1[k] = nums2[j];
            j = j.wrapping_sub(1);
            k = k.wrapping_sub(1);
        }
    }
}