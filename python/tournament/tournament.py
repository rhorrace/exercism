from collections import OrderedDict


def tally(rows):
    teams = {}
    table = ["Team                           | MP |  W |  D |  L |  P"]
    for row in rows:
        info = row.split(";")
        if info[0] not in teams:
            teams[info[0]] = [0] * 5
        if info[1] not in teams:
            teams[info[1]] = [0] * 5
        if info[2] == "win":
            teams[info[0]][0] += 1
            teams[info[0]][1] += 1
            teams[info[0]][4] += 3
            teams[info[1]][0] += 1
            teams[info[1]][3] += 1
        elif info[2] == "loss":
            teams[info[1]][0] += 1
            teams[info[1]][1] += 1
            teams[info[1]][4] += 3
            teams[info[0]][0] += 1
            teams[info[0]][3] += 1
        elif info[2] == "draw":
            teams[info[0]][0] += 1
            teams[info[1]][0] += 1
            teams[info[0]][2] += 1
            teams[info[1]][2] += 1
            teams[info[0]][4] += 1
            teams[info[1]][4] += 1
    teams_info = sorted(teams.items(), key=lambda item: (-item[1][4], item[0]))
    for team, info in teams_info:
        tallies = "| {:2d} | {:2d} | {:2d} | {:2d} | {:2d}".format(info[0], info[1],
                                                                   info[2], info[3],
                                                                   info[4])
        table.append("{:31s}{}".format(team, tallies))
    return table
