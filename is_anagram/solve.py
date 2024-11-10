class Solution:
    def isAnagram(self, s: str, t: str) -> bool:
        if s and t:
            return sorted(s) == sorted(t)
        
if __name__ == '__main__':
    solution = Solution()
    s = "racecar"
    t = "carrace"
    print(solution.isAnagram(s,t))
    print(solution.isAnagram("jar","jam"))