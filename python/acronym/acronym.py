def abbreviate(words):
    acronym = ""
    letters = []
    for letter in words.lower():
        if letter.isalnum() or letter == "'":
            letters += letter
        else:
            letters += " "
    words = "".join(letters).split()
    for word in words:
        acronym += word[0]
    return acronym.upper()
