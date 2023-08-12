class Solution {
public:
    int majorityElement(vector<int>& nums) {

        //it works only because i know for a fact that the number occures more than n/2
        int count = 0, num;
        for(int val : nums){
            if(count == 0)
                num = val;
            
            if(num == val)
                count++;
            else 
                count --;
        }

        return num;
    }
};