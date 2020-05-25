def to_rna(dna_strand):
    return "".join(convert(c) for c in dna_strand)

def convert(c):
    if c == 'A':
        return 'U'
    elif c == 'T':
        return 'A'
    elif c == 'C':
        return 'G'
    elif c == 'G':
        return 'C'
    else:
        raise ValueError("Error: Not a nucleotide")