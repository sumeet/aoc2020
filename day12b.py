ns = 7
c = 0

for mod, eq in [[13, 1]]:
    ns *= mod
    l = (eq * -11) % mod
    print(f'l = {l}')
    c += l


print(f'{ns} - {c}')
