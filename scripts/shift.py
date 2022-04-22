shift = {}
shift['`'] = '~'
shift['1'] = '!'
shift['2'] = '@'
shift['3'] = '#'
shift['4'] = '$'
shift['5'] = '%'
shift['6'] = '^'
shift['7'] = '&'
shift['8'] = '*'
shift['9'] = '('
shift['0'] = ')'
shift['-'] = '_'
shift['='] = '+'
shift['['] = '{'
shift[']'] = '}'
shift['\\'] = '|'
shift[';'] = ':'
shift["'"] = '"'
shift[','] = '<'
shift['.'] = '>'
shift['/'] = '?'

print("match qwerty_press {")
for qwerty_press in shift:
    print(f"{ord(qwerty_press)} => {ord(shift[qwerty_press])},")
print("_ => qwerty_press")
print("}")
