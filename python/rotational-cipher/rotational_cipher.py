def rotate(text, key):
    return "".join(list(map(lambda c: cipher(c, key), list(text))))


def cipher(letter, key):
    letters = "abcdefghijklmnopqrstuvwxyz"
    low_letter_to_number = dict(zip(list(letters), range(0, 26)))
    up_letter_to_number = dict(zip(list(letters.upper()), range(0, 26)))
    number_to_low_letter = {v: k for k, v in low_letter_to_number.items()}
    number_to_up_letter = {v: k for k, v in up_letter_to_number.items()}
    if letter in low_letter_to_number:
        number = low_letter_to_number[letter]
        number = (number + key) % 26
        return number_to_low_letter[number]
    elif letter in up_letter_to_number:
        number = up_letter_to_number[letter]
        number = (number + key) % 26
        return number_to_up_letter[number]
    else:
        return letter
