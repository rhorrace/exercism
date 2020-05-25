def find_anagrams(word, candidates):
    word_lower = word.lower()
    sorted_letters = sorted(word_lower)
    anagrams = []
    for candidate in candidates:
        candidate_lower = candidate.lower()
        if sorted(candidate_lower) == sorted_letters:
            if candidate_lower != word_lower:
                anagrams.append(candidate)
    return anagrams
