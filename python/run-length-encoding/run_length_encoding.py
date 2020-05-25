def decode(string):
    decoded = ""
    number = ""
    for c in string:
        if c.isdigit():
            number += c
        else:
            if number == "":
                decoded += c
            else:
                decoded += int(number) * c
                number = ""
    return decoded


def encode(string):
    letters = []

    for c in string:
        if len(letters) == 0 or c != letters[-1][-1]:
            letters.append([c])
        else:
            letters[-1].append(c)

    return "".join(map(convert, letters))

def convert(letter_list):
    size = len(letter_list)
    if size == 1:
        return letter_list[0]
    else:
        return "{}{}".format(size, letter_list[0])