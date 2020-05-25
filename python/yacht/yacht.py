"""
This exercise stub and the test suite contain several enumerated constants.

Since Python 2 does not have the enum module, the idiomatic way to write
enumerated constants has traditionally been a NAME assigned to an arbitrary,
but unique value. An integer is traditionally used because it’s memory
efficient.
It is a common practice to export both constants and functions that work with
those constants (ex. the constants in the os, subprocess and re modules).

You can learn more here: https://en.wikipedia.org/wiki/Enumerated_type
"""

# Score categories.
# Change the values as you see fit.

ONES = 1
TWOS = 2
THREES = 3
FOURS = 4
FIVES = 5
SIXES = 6
FULL_HOUSE = 7
FOUR_OF_A_KIND = 8
LITTLE_STRAIGHT = 9
BIG_STRAIGHT = 10
CHOICE = 11
YACHT = 12


def score(dice, category):
    if category == YACHT:
        return get_yacht_score(dice)
    elif category == CHOICE:
        return get_choice(dice)
    elif category == BIG_STRAIGHT:
        return get_straight(dice, [2, 3, 4, 5, 6])
    elif category == LITTLE_STRAIGHT:
        return get_straight(dice, [1, 2, 3, 4, 5])
    elif category == FOUR_OF_A_KIND:
        return get_four_kind(dice)
    elif category == FULL_HOUSE:
        return get_full_house(dice)
    elif category == SIXES:
        return get_count(dice, 6)
    elif category == FIVES:
        return get_count(dice, 5)
    elif category == FOURS:
        return get_count(dice, 4)
    elif category == THREES:
        return get_count(dice, 3)
    elif category == TWOS:
        return get_count(dice, 2)
    elif category == ONES:
        return get_count(dice, 1)
    else:
        raise ValueError("Error: Not Valid")


def get_yacht_score(dice):
    return 50 if all(die == dice[0] for die in dice) else 0


def get_choice(dice):
    return sum(dice)


def get_count(dice, number):
    return number * dice.count(number)


def get_straight(dice, pattern):
    return 30 if list(sorted(dice)) == pattern else 0


def get_four_kind(dice):
    freq = set_freq(dice)
    for k, v in freq.items():
        if v >= 4:
            return k * 4
    return 0


def get_full_house(dice):
    freq = set_freq(dice)
    values = freq.values()
    if 2 in values and 3 in values:
        return sum(k * v for k, v in freq.items())
    return 0


def set_freq(dice):
    freq = {}
    for die in dice:
        freq[die] = dice.count(die)
    return freq
