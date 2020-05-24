class Luhn:
    def __init__(self, card_num):
        self.__numbers = list(card_num.replace(" ", ""))

    def valid(self):
        if any(map(lambda n: not n.isdigit(), self.__numbers)):
            return False
        elif len(self.__numbers) <= 1:
            return False
        numbers = list(reversed(list(map(int, self.__numbers))))
        for i in range(1, len(numbers), 2):
            numbers[i] *= 2
            if numbers[i] > 9:
                numbers[i] -= 9
        return sum(numbers) % 10 == 0
