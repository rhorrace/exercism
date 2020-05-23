def distance(strand_a, strand_b):
    if len(strand_a) != len(strand_b):
        raise ValueError("Needs to be same length")
    count = 0
    for i,j in zip(strand_a, strand_b):
        if i != j:
            count += 1
    return count
