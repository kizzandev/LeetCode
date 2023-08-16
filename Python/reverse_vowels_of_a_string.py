# Solution
class Solution:
    def reverseVowels(self, s: str) -> str:
        vowels = 'aeiouAEIOU'
        s_lst = list(s)
        left, right = 0, len(s_lst) - 1
        
        while left < right:
            while left < right and s_lst[left] not in vowels:
                left += 1
            while left < right and s_lst[right] not in vowels:
                right -= 1

            if left < right:
                s_lst[left], s_lst[right] = s_lst[right], s_lst[left]
                left += 1
                right -= 1

        return ''.join(s_lst)
