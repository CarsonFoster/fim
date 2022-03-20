dvorak = {}
dvorak['q'] = "'" 
dvorak['w'] = ','
dvorak['e'] = '.'
dvorak['r'] = 'p'
dvorak['t'] = 'y'
dvorak['y'] = 'f'
dvorak['u'] = 'g'
dvorak['i'] = 'c'
dvorak['o'] = 'r'
dvorak['p'] = 'l'
dvorak['['] = '/'
dvorak[']'] = '='
dvorak['a'] = 'a'
dvorak['s'] = 'o'
dvorak['d'] = 'e'
dvorak['f'] = 'u'
dvorak['g'] = 'i'
dvorak['h'] = 'd'
dvorak['j'] = 'h'
dvorak['k'] = 't'
dvorak['l'] = 'n'
dvorak[';'] = 's'
dvorak["'"] = '-'
dvorak['z'] = ';' 
dvorak['x'] = 'q'
dvorak['c'] = 'j'
dvorak['v'] = 'k'
dvorak['b'] = 'x'
dvorak['n'] = 'b'
dvorak['m'] = 'm'
dvorak[','] = 'w'
dvorak['.'] = 'v'
dvorak['/'] = 'z'
dvorak['-'] = '['
dvorak['='] = ']'

print("match qwerty_press {")
for qwerty_press in dvorak:
    print(f"    {ord(qwerty_press)} => {ord(dvorak[qwerty_press])},")
print("    _ => qwerty_press")
print("}")
