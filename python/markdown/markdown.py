import re


def parse(markdown):
    lines = markdown.split('\n')
    res = ''
    in_list = False
    for i in lines:
        in_list_append = False

        if i.startswith('######'):
            i = '<h6>' + i[7:] + '</h6>'
        elif i.startswith('##'):
            i = '<h2>' + i[3:] + '</h2>'
        elif i.startswith('#'):
            i = '<h1>' + i[2:] + '</h1>'
        elif i.startswith('*'):
            i, in_list = get_list(i[2:], in_list)
        else:
            if in_list:
                in_list_append = True
                in_list = False

        if not re.match('<h|<ul|<p|<li', i):
            i = '<p>' + i + '</p>'
        i = get_bold_and_italic(i)
        if in_list_append:
            i = '</ul>' + i
        res += i
    if in_list:
        res += '</ul>'
    return res


def get_bold_and_italic(i):
    m = re.match('(.*)__(.*)__(.*)', i)
    if m:
        i = boldify(m)
    m = re.match('(.*)_(.*)_(.*)', i)
    if m:
        i = italicize(m)
    return i


def get_list(i, in_list):
    i = '<li>' + get_bold_and_italic(i) + '</li>'
    if not in_list:
        in_list = True
        i = '<ul>' + i
    return (i, in_list)


def boldify(m):
    return m.group(1) + '<strong>' + \
           m.group(2) + '</strong>' + m.group(3)


def italicize(m):
    return m.group(1) + '<em>' + \
           m.group(2) + '</em>' + m.group(3)
