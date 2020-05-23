def recite(start_verse, end_verse):
    verses = []
    for i in range(start_verse, end_verse + 1):
        current_verse = verse(i)
        verses.append(current_verse)
    return verses

def verse(n):
    days = {1: "first", 2: "second", 3: "third", 4: "fourth",
            5: "fifth", 6: "sixth", 7: "seventh", 8: "eighth",
            9: "ninth", 10: "tenth", 11: "eleventh", 12: "twelfth"}
    line_numbers = {1: "a Partridge in a Pear Tree.",
                    2: "two Turtle Doves, ",
                    3: "three French Hens, ",
                    4: "four Calling Birds, ",
                    5: "five Gold Rings, ",
                    6: "six Geese-a-Laying, ",
                    7: "seven Swans-a-Swimming, ",
                    8: "eight Maids-a-Milking, ",
                    9: "nine Ladies Dancing, ",
                    10: "ten Lords-a-Leaping, ",
                    11: "eleven Pipers Piping, ",
                    12: "twelve Drummers Drumming, "}
    song = ["On the {0} day of Christmas my true love gave to me: ".format(days[n])]
    lines = []
    for i in reversed(range(1, n + 1)):
        lines.append(line_numbers[i])
    if len(lines) > 1:
        lines[-1] = "and " + lines[-1]
    return "".join(song + lines)
