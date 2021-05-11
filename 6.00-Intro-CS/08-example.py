
def f(x):
    k = 100
    print(k)
    def g():
        nonlocal k
        k += 1
    g()
    print(k)
    x = x + 1
    return x

def g(x):
    x = x + 1
    return x

x = -3
print(f(x))
print(x)
print(g(x))


