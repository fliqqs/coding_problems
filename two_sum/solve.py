from typing import List, Dict

class Solution:
    def twoSum(self, nums: List[int], target: int) -> List[int]:
        compliments: Dict = {}
        for i, num in enumerate(nums):
            compliment = target - num
            if compliment in compliments:
                return [compliments[compliment], i]
            compliments[num] = i

if __name__ == '__main__':
    solution = Solution()
    nums = [3,2,4]
    target = 6
    print(solution.twoSum(nums, target))  # [1, 2]