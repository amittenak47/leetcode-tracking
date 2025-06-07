# Last updated: 6/7/2025, 2:41:50 AM
class Solution:
    def moveZeroes(self, nums: List[int]) -> None:
        """
        Do not return anything, modify nums in-place instead.
        """
        last_non_zero_index = 0
        
        for i in nums:
            if i != 0:
                nums[last_non_zero_index] = i
                last_non_zero_index += 1
        
        for i in range(last_non_zero_index, len(nums)):
            nums[i] = 0