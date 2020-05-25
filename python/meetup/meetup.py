from datetime import date
from calendar import monthrange, weekday


class MeetupDayException(ValueError):
    def __init__(self, exception):
        super(exception)


def meetup(year, month, week, day_of_week):
    days = {"Monday": 0, "Tuesday": 1,
            "Wednesday": 2, "Thursday": 3,
            "Friday": 4, "Saturday": 5,
            "Sunday": 6}
    num_day_of_week = days[day_of_week]
    first_day, num_days = monthrange(year, month)
    week_order = {"1st": 0, "2nd": 1,
                  "3rd": 2, "4th": 3,
                  "5th": 4}

    if week in week_order:
        day = calculate(num_day_of_week, first_day, week_order[week])
    elif week == "last":
        day = num_days - ((weekday(year, month, num_days) - num_day_of_week) % 7)
    elif week == "teenth":
        week_with_13 = weekday(year, month, 13)
        day = ((num_day_of_week - week_with_13) % 7) + 13
    else:
        raise MeetupDayException("week is not valid")

    try:
        return date(year, month, day)
    except ValueError as e:
        raise MeetupDayException(str(e))


def calculate(num_day_of_week, first_day, week_order):
    return ((num_day_of_week - first_day) % 7) + 1 + (week_order * 7)
