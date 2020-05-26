class Allergies:
    __allergies_dict = {1: "eggs",
                        2: "peanuts",
                        4: "shellfish",
                        8: "strawberries",
                        16: "tomatoes",
                        32: "chocolate",
                        64: "pollen",
                        128: "cats"}

    def __init__(self, score):
        self.__allergens = []
        score %= 256
        for i in [128, 64, 32, 16, 8, 4, 2, 1]:
            if score >= i:
                self.__allergens.append(self.__allergies_dict[i])
                score -= i
        self.__allergens = list(reversed(self.__allergens))

    def allergic_to(self, item):
        return item in self.__allergens

    @property
    def lst(self):
        return self.__allergens
