a = {2: 'd', 4: 'i'}
a.update([(i, 'h') for i in [3, 6, 16, 22, 23]])
a.update([(i, 'c') for i in [1, 5, 15]])

print(a)
print()

print(a[2])
print()

print(a[4])
print()

print(a[6])
print()

for key in a:
    print(key, a[key])