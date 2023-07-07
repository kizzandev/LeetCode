# Added
from typing import List

# Solution
class Solution:
    def kidsWithCandies(self, candies: List[int], extraCandies: int) -> List[bool]:
        # Max value in candies
        greatest = max(candies)
        # The following is a 'generator expression'
        # Note the difference with list comprehensions: parenthesis instead of square brackets
        # This creates a generator without the yield keyword
        # In a real-world scenario, it is recommended to convert the generator object into a list
        # The above will ensure that the return type will match to what's actually returned
        # Either use "[]" or list((candy + extraCandies >= greatest for candy in candies))
        return (candy + extraCandies >= greatest for candy in candies)
