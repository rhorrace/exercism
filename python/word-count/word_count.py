def count_words(sentence):
    word_count = {}
    letters = []
    for letter in sentence.lower():
        if letter.isalnum() or letter == "'":
            letters += letter
        else:
            letters += " "
    words = "".join(letters).split()
    for word in words:
        word = word.strip("'")
        if word in word_count:
            word_count[word] += 1
        else:
            word_count[word] = 1
    return word_count
