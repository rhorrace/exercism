def is_valid(isbn):
    numbers = [n for n in isbn if n.isalnum()]
    if len(numbers) != 10:
        return False
    elif numbers[-1] not in "012345689X":
        return False
    elif any(not n.isdigit() for n in numbers[:-1]):
        return False
    numbers.reverse()
    numbers = list(map(convert, numbers))
    total = sum((i + 1) * n for i, n in enumerate(numbers))
    return total % 11 == 0


def convert(n):
    return int(n) if n != 'X' else 10
