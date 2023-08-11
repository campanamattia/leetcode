class Solution {
public:
    int removeDuplicates(vector<int>& nums) {
        int count = 0;
        for(int i = 0; i < nums.size(); i++){
            int k = i + 1;
            while(k < nums.size() && nums[k] == nums[i])
                k++;

            if(k == i + 1){
                nums[count] = nums[i];
                count++;
                continue;
            }

            nums[count] = nums[i], count++;
            nums[count] = nums[i], count++;

            i = k -1;
        }
        return count;
    }
};