def maxArea(heights):
    """
    :type heights: List[int]
    :rtype: int
    """
    left, right = 0, len(heights) - 1
    max_area = 0

    while left < right:
        area = min(heights[left], heights[right]) * (right - left)

        if area > max_area:
            max_area = area

        if heights[left] < heights[right]:
            left += 1
        else:
            right -= 1

    return max_area


max = maxArea([1, 8, 6, 2, 5, 4, 8, 3, 7])
print(max)
