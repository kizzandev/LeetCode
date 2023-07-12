# Added
from typing import List

# Solution
class Solution:
    def canPlaceFlowers(self, flowerbed: List[int], n: int) -> bool:
        if n == 0:
            return True

        count = skip = 0

        for i in range(len(flowerbed) - 1, -1, -1):
            if flowerbed[i] == 1:
                skip = 1
            else:
                if skip == 0 and (i == 0 or flowerbed[i-1] == 0):
                    count += 1
                    skip = 1
                else:
                    skip = 0

            if count >= n:
                return True

        return count >= n

# Test
if __name__ == '__main__':
    flowerbed = [1, 0, 0, 0, 1]
    n = 1
    print(Solution().canPlaceFlowers(flowerbed, n))

    flowerbed = [1, 0, 0, 0, 1]
    n = 2
    print(Solution().canPlaceFlowers(flowerbed, n))

    flowerbed = [1, 0, 0, 0, 0, 1]
    n = 2
    print(Solution().canPlaceFlowers(flowerbed, n))

    flowerbed = [0, 0, 1, 0, 1]
    n = 1
    print(Solution().canPlaceFlowers(flowerbed, n))

    flowerbed = [1, 0, 0, 0, 1, 0, 0]
    n = 2
    print(Solution().canPlaceFlowers(flowerbed, n))

    flowerbed = [0, 0, 0, 0, 1, 0, 0]
    n = 3
    print(Solution().canPlaceFlowers(flowerbed, n))

    flowerbed = [0]
    n = 1
    print(Solution().canPlaceFlowers(flowerbed, n))

    flowerbed = [0, 1, 0]
    n = 1
    print(Solution().canPlaceFlowers(flowerbed, n))
