
# Bisection approach to finding Square Root
x = 25
epsilon = 0.01
count = 0
lo, hi = 1.0, x
guess = lo + (hi - lo) / 2.0

while abs(guess**2 - x) >= epsilon:
    if guess**2 < x:
        lo = guess
    else:
        hi = guess
    guess = lo + (hi - lo) / 2.0
    count += 1

print(f'It has taken {count} number of steps')
print(f'{guess} is close to square root of {x}')

# Iterative approach to finding Square root
#x = int(input("Enter an integer: "))
#
#epsilon = 0.01
#guess = 0.0
#increment = 0.0001
#count = 0
#
#while abs(guess**3 - x) >= epsilon and guess <= x:
#    guess += increment
#    count += 1
#
#print(f"It has taken {count} number of steps.")
#
#if abs(guess**3 - x) >= epsilon:
#    print(f"Cannot find the cube root of {x}.")
#else:
#    print(f"{guess} is close to the cube root of {x}.")
