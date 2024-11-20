from typing import List

class Solution:
    def max_profit(self, prices: List[int]) -> int:
        max_profit = 0
        lowest_buy = prices[0]

        for p in prices:
            possible_profit = p - lowest_buy
            max_profit = max(max_profit, possible_profit)
            lowest_buy = min(lowest_buy, p)
        return max_profit
    
if __name__ == '__main__':
    solution = Solution()
    prices = [10,1,5,6,7,1]
    assert(solution.max_profit(prices) == 6)