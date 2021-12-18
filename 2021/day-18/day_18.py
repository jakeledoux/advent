from itertools import combinations
import math
import re

with open("input.txt", "r") as f:
    lines = [line.strip() for line in f.readlines()]


pair_pattern = re.compile(r"\[(\d+),(\d+)\]")
end_digit_pattern = re.compile(r"[,\]]")


def find_and_add(expr, start, inc, n):
    pos = start
    while pos in range(0, len(expr)):
        char = expr[pos]
        if char.isdigit():
            start = pos
            while char.isdigit():
                pos += inc
                char = expr[pos]
            end = pos
            if inc > 0:
                num = int(expr[start:end])
                return expr[:start] + str(num + n) + expr[end:]
            else:
                num = int(expr[end + 1 : start + 1])
                return expr[: end + 1] + str(num + n) + expr[start + 1 :]
        pos += inc
    return expr


def reduce(expr, verbose=False):
    prev_expr = ""
    while prev_expr != expr:
        prev_expr = expr
        expr = explode(expr, verbose=verbose)
        expr = split(expr, verbose=verbose)
    return expr


def split(expr, recurse=True, verbose=False):
    depth = 0
    pos = 0

    prev_char = ""
    while True:
        char = expr[pos]
        depth += {"[": 1, "]": -1}.get(char, 0)

        if char.isdigit() and not prev_char.isdigit():
            digit = int(end_digit_pattern.split(expr[pos:])[0])
            if digit >= 10:
                f, c = math.floor(digit / 2), math.ceil(digit / 2)
                expr = expr[:pos] + f"[{f},{c}]" + expr[pos + len(str(digit)) :]
                if verbose:
                    print("after split:", expr)

                # Explode any new problems
                expr = explode(expr, verbose=verbose)

                if recurse:
                    return split(expr, verbose=verbose)
                else:
                    return expr

        prev_char = char

        pos += 1
        if pos == len(expr):
            return expr


def explode(expr, recurse=True, verbose=False):
    depth = 0
    pos = 0

    while True:
        char = expr[pos]
        depth += {"[": 1, "]": -1}.get(char, 0)

        if depth == 5:
            start, end = pos, expr.find("]", pos) + 1
            left, right = [int(n) for n in pair_pattern.match(expr[start:end]).groups()]
            expr = expr[:start] + "0" + expr[end:]
            depth -= 1

            expr = find_and_add(expr, pos - 1, -1, left)
            expr = find_and_add(expr, pos + 2, 1, right)

            if verbose:
                print("after explode:", expr)
            if not recurse:
                return expr
            return explode(expr, verbose=verbose)

        pos += 1
        if pos == len(expr):
            return expr


def add(*args, verbose=False):
    nums = list(args)
    a = nums.pop(0)
    while nums:
        if verbose:
            print("Next number...")
        b = nums.pop(0)
        a = reduce(f"[{a},{b}]", verbose=verbose)
    return a


def magnitude(num):
    if isinstance(num, str):
        return magnitude(eval(num))
    elif isinstance(num, int):
        return num
    else:
        return magnitude(num[0]) * 3 + magnitude(num[1]) * 2


def part_one():
    return magnitude(add(*lines))


def part_two():
    magnitudes = [magnitude(add(a, b)) for a, b in combinations(lines, 2)]
    return max(magnitudes)


if __name__ == "__main__":
    print(f"Part one: {part_one()}")
    print(f"Part two: {part_two()}")


def test_explode():
    cases = [
        ("[[[[[9,8],1],2],3],4]", "[[[[0,9],2],3],4]"),
        ("[7,[6,[5,[4,[3,2]]]]]", "[7,[6,[5,[7,0]]]]"),
        ("[[6,[5,[4,[3,2]]]],1]", "[[6,[5,[7,0]]],3]"),
        ("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]", "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]"),
        ("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]", "[[3,[2,[8,0]]],[9,[5,[7,0]]]]"),
        ("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]", "[[3,[2,[8,0]]],[9,[5,[7,0]]]]"),
        ("[[3[[[10,10],0],0]],0]", "[[13[[0,10],0]],0]"),
        ("[[[[0,[23,40]],0],0],0]", "[[[[23,0],40],0],0]"),
        ("[[[[0,[0,40]],0],0],0]", "[[[[0,0],40],0],0]"),
        ("[[[[0,[30,0]],0],0],0]", "[[[[30,0],0],0],0]"),
        (
            "[[[[4,0],[5,4]],[[7,7],[6,0]]],[[[[10,10],20],40],[[11,9],[11,0]]]]",
            "[[[[4,0],[5,4]],[[7,7],[6,10]]],[[[0,30],40],[[11,9],[11,0]]]]",
        ),
    ]

    for prompt, expected in cases:
        assert explode(prompt, recurse=False) == expected


def test_split():
    cases = [("[50,1]", "[[25,25],1]"), ("[0,100]", "[0,[50,50]]")]

    for prompt, expected in cases:
        assert split(prompt, recurse=False, verbose=True) == expected


def test_reduce():
    cases = [
        ("[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]", "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]"),
    ]

    for prompt, expected in cases:
        assert reduce(prompt) == expected


def test_add():
    cases = [
        (
            ("[[[[4,3],4],4],[7,[[8,4],9]]]", "[1,1]"),
            "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]",
        ),
        (("[1,1]", "[2,2]", "[3,3]", "[4,4]"), "[[[[1,1],[2,2]],[3,3]],[4,4]]"),
        (
            ("[1,1]", "[2,2]", "[3,3]", "[4,4]", "[5,5]", "[6,6]"),
            "[[[[5,0],[7,4]],[5,5]],[6,6]]",
        ),
        (
            (
                "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]",
                "[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]",
            ),
            "[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]",
        ),
        (
            (
                "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]",
                "[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]",
                "[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]",
                "[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]",
                "[7,[5,[[3,8],[1,4]]]]",
                "[[2,[2,2]],[8,[8,1]]]",
                "[2,9]",
                "[1,[[[9,3],9],[[9,0],[0,7]]]]",
                "[[[5,[7,4]],7],1]",
                "[[[[4,2],2],6],[8,7]]",
            ),
            "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]",
        ),
    ]

    for nums, expected in cases:
        assert add(*nums, verbose=True) == expected


def test_magnitude():
    case = [
        ("[[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]", 4140),
        ("[[1,2],[[3,4],5]]", 143),
        ("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]", 1384),
        ("[[[[1,1],[2,2]],[3,3]],[4,4]]", 445),
        ("[[[[3,0],[5,3]],[4,4]],[5,5]]", 791),
        ("[[[[5,0],[7,4]],[5,5]],[6,6]]", 1137),
        ("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]", 3488),
    ]

    for num, expected in case:
        assert magnitude(num) == expected
