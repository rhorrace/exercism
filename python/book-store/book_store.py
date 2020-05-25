def total(basket):
    discounts = {1: 1, 2: 0.95, 3: 0.9, 4: 0.8, 5: 0.75}
    groups = []
    for book in basket:
        added = False
        for group in groups:
            if book not in group:
                group.append(book)
                added = True
                break
        if not added:
            groups.append([book])
            continue

    totals = list(map(lambda group: 800 * len(group) * discounts[len(group)], groups))
    total = sum(totals)

    groups_len = [len(group) for group in groups]
    combinable = min(groups_len.count(5), groups_len.count(3))

    total -= 40 * combinable

    return total
