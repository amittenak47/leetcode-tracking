# Last updated: 6/6/2025, 3:31:49 PM
class Solution:
    def smallestDistancePair(self, nums: List[int], k: int) -> int:
        
        nums.sort()

        def helper(dist):
            l, res = 0, 0
            for r in range(len(nums)):
                while nums[r] - nums[l] > dist:
                    l += 1
                res += r - l
                if res >= k:
                    break
            return res

        l, r = 0, max(nums)
        while l < r:
            m = l + ((r - l) // 2)
            pairs = helper(m)
            if pairs >= k:
                r = m
            else:
                l = m + 1
        return r
