def is_isogram(string):
    visited = set()
    for c in string.lower():
        if not c.isalpha():
            continue
        if c in visited:
            return False
        visited.add(c)
    return True
