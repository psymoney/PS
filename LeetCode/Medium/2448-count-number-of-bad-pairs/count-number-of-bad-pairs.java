class Solution {
    public long countBadPairs(int[] nums) {
        int n = nums.length;
        long count = 0;
        HashMap<Integer, Integer> frequency = new HashMap();
        
        for (int i = 0; i < n; i++) {
            count += -1 + frequency.merge(i - nums[i], 1, Integer::sum);
        }

        return 1L * n * (n - 1) / 2 - count;
    }
}