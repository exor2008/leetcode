class Solution(object):
    def jump(self, nums):
        """
        :type nums: List[int]
        :rtype: int
        """
        nums = [(i, n, i + n) for i, n in enumerate(nums)]
        idx = n_jumps = 0
        l = len(nums)

        if l == 1:
            return 0
        elif l == 2:
            return 1

        if nums[idx][2] >= l - 1:
            return 1

        while True:
            _, num, jump = nums[idx]

            if num == 1:
                idx += 1
                _, num, jump = nums[idx]
            else:
                idx, num, jump = max(nums[idx + 1 : jump + 1], key=lambda nj: nj[2])

            n_jumps += 1

            if jump >= l - 1:
                if idx == l - 1:
                    return n_jumps
                else:
                    return n_jumps + 1


if __name__ == "__main__":
    print(Solution().jump([1, 1, 1, 1]))
