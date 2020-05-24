class School:
    def __init__(self):
        self.__students = {}

    def add_student(self, name, grade):
        if grade not in self.__students:
            self.__students[grade] = [name]
        else:
            self.__students[grade].append(name)
            self.__students[grade] = sorted(self.__students[grade])

    def roster(self):
        students = []
        for key in sorted(self.__students.keys()):
            students += self.__students[key]
        return students

    def grade(self, grade_number):
        if grade_number not in self.__students:
            return []
        else:
            return self.__students[grade_number]
