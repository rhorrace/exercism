def is_armstrong_number(number):
    digits = list(map(int, (list(str(number)))))
    n = len(digits)
    return sum(map(lambda d: d ** n, digits)) == number
