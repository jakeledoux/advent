import math
import re

with open("sample.txt", "r") as f:
    lines = [line.strip() for line in f.readlines()]


pair_pattern = re.compile(r"\[(\d+),(\d+)\]")
end_digit_pattern = re.compile(r"[,\]]")


def find_and_add(expr, start, inc, n):
    pos = start
    while pos in range(0, len(expr)):
        char = expr[pos]
        if char.isdigit():
            return expr[:pos] + str(int(char) + n) + expr[pos + 1 :]
        pos += inc
    return expr


def reduce(expr, explode_once=False, verbose=False):
    depth = 0
    pos = 0

    prev_char = ""
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

            if explode_once:
                return expr
            if verbose:
                print("after explode:", expr)
            return reduce(expr, verbose=verbose)

        elif char.isdigit() and not prev_char.isdigit():
            digit = int(end_digit_pattern.split(expr[pos:])[0])
            if digit >= 10:
                f, c = math.floor(digit / 2), math.ceil(digit / 2)
                expr = expr[:pos] + f"[{f},{c}]" + expr[pos + len(str(digit)) :]
                if verbose:
                    print("after split:", expr)

                expr = reduce(expr, explode_once=True, verbose=verbose)
                return reduce(expr, verbose=verbose)

        prev_char = char

        pos += 1
        if pos == len(expr):
            return expr



def add(*args):
    nums = list(args)
    a = nums.pop(0)
    while nums:
        b = nums.pop(0)
        a = reduce(f"[{a},{b}]", verbose=True)
    return a


if __name__ == "__main__":
    pass


def test_explode():
    cases = [
        ("[[[[[9,8],1],2],3],4]", "[[[[0,9],2],3],4]"),
        ("[7,[6,[5,[4,[3,2]]]]]", "[7,[6,[5,[7,0]]]]"),
        ("[[6,[5,[4,[3,2]]]],1]", "[[6,[5,[7,0]]],3]"),
        ("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]", "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]"),
        ("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]", "[[3,[2,[8,0]]],[9,[5,[7,0]]]]"),
    ]

    for prompt, expected in cases:
        assert reduce(prompt, explode_once=True) == expected


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
            (
                "[1,1]",
                "[2,2]",
                "[3,3]",
                "[4,4]",
                "[5,5]",
                "[6,6]"
            ),
            "[[[[5,0],[7,4]],[5,5]],[6,6]]"
        ),
        (
            (
                "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]",
                "[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]"
            ),
            "[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]"
        ),
        # (
        #     (
        #         "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]",
        #         "[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]",
        #         "[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]",
        #         "[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]",
        #         "[7,[5,[[3,8],[1,4]]]]",
        #         "[[2,[2,2]],[8,[8,1]]]",
        #         "[2,9]",
        #         "[1,[[[9,3],9],[[9,0],[0,7]]]]",
        #         "[[[5,[7,4]],7],1]",
        #         "[[[[4,2],2],6],[8,7]]",
        #     ),
        #     "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]",
        # ),
    ]

    for nums, expected in cases:
        assert add(*nums) == expected
