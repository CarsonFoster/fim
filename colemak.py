colemak = {}
colemak['e'] = 'f'
colemak['r'] = 'p'
colemak['t'] = 'g'
colemak['y'] = 'j'
colemak['u'] = 'l'
colemak['i'] = 'u'
colemak['o'] = 'y'
colemak['p'] = ';'
colemak['s'] = 'r'
colemak['d'] = 's'
colemak['f'] = 't'
colemak['g'] = 'd'
colemak['j'] = 'n'
colemak['k'] = 'e'
colemak['l'] = 'i'
colemak[';'] = 'o'
colemak['n'] = 'k'

print("match qwerty_press {")
for qwerty_press, colemak_press in colemak.items():
    print(f"{ord(qwerty_press)} => {ord(colemak_press)},")
print("other => other")
print("}")
