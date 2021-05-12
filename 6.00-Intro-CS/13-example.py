



li1 = [1, 2, 3]
li2 = li1

print(li1, li2)
print(id(li1), id(li2))

li1.append(99)

print(li1, li2)

li3 = [1, 2, 3]

print(li1 == li2)
print(li1 == li3)
