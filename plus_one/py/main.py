class Solution(object):
    def plusOne(self, digits):
        """
        :type digits: List[int]
        :rtype: List[int]
        """
        dig = int("".join(map(str, digits)))
        return list(map(int, str(dig + 1)))


if __name__ == "__main__":
    assert Solution().plusOne([1, 2, 4, 9]) == [1, 2, 5, 0]
