class Solution {
public:
    int removeDuplicates(vector<int>& nums) {
        int count = 0;
        for(int i = 0; i < nums.size(); i++){

            int k = i + 1;
            while(k < nums.size() && nums[k] == nums[i])
                k++;

            nums[count] = nums[i];
            i = k - 1;
            count++;   
        }
        return count;
    }
};