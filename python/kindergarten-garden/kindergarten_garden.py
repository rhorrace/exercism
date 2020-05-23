class Garden:
    __default_students = ["Alice", "Bob", "Charlie",
                          "David", "Eve", "Fred",
                          "Ginny", "Harriet", "Ileana",
                          "Joseph", "Kincaid", "Larry"]
    __plant_dict = {'R': "Radishes", 'C': "Clover",
                    'G': "Grass", 'V': "Violets"}

    def __init__(self, diagram, students=__default_students):
        plants_1, plants_2 = diagram.split()
        self.__student_plants = {}
        students = sorted(students)
        for i, student in enumerate(students):
            try:
                self.__student_plants[student] = [plants_1[2 * i], plants_1[2 * i + 1],
                                                  plants_2[2 * i], plants_2[2 * i + 1]]
            except IndexError:
                break

    def plants(self, name):
        return list(map(lambda s: self.__plant_dict[s], self.__student_plants[name]))
