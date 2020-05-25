def is_pangram(sentence):
    return len(set(c for c in sentence.lower() if c.isalpha())) == 26
