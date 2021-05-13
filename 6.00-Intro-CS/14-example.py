

def doForEach(l, f):
    for i in range(len(l)):
        l[i] = f(l[i])

def multByTwo(e):
    return e * 2


li = [1, 2, 3, 4, 5]

print(li)

doForEach(li, multByTwo)

print(li)
