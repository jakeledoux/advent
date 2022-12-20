with open("input.txt", "r") as f:
    lines = f.readlines()

from collections import Counter

print(Counter(lines).most_common(3))
