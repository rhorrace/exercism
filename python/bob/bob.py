def response(hey_bob):
    phrase = hey_bob.strip();
    has_alpha = any(c.isalpha() for c in phrase)
    if phrase == "":
        return "Fine. Be that way!"
    elif phrase[-1] == '?':
        if phrase == phrase.upper() and has_alpha:
            return "Calm down, I know what I'm doing!"
        else:
            return "Sure."
    elif phrase == phrase.upper() and has_alpha:
        return "Whoa, chill out!"
    else:
        return "Whatever."
