
class Matrix:
    def __init__(self, matrix_string):
        self.__matrix = []
        rows = matrix_string.split('\n')
        for row in rows:
            numbers = row.split(' ')
            numbers = list(map(int, numbers))
            self.__matrix.append(numbers)
        print(self.__matrix)

    def row(self, index):
        return self.__matrix[index - 1]

    def column(self, index):
        return list(map(lambda row: row[index - 1], self.__matrix))
