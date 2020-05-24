class Clock:
    def __init__(self, hour, minute):
        self.__total_minutes = (hour * 60 + minute) % 1440

    def __repr__(self):
        hour = (self.__total_minutes // 60) % 24
        minute = self.__total_minutes % 60
        return "{:02d}:{:02d}".format(hour, minute)

    def __eq__(self, other):
        return self.__total_minutes == other.__total_minutes

    def __add__(self, minutes):
        return Clock(0, self.__total_minutes + minutes)

    def __sub__(self, minutes):
        return Clock(0, self.__total_minutes - minutes)
