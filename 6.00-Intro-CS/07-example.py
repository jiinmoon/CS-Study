# Convert to binary format
x = int(input("Enter an integer: "))

sign = x < 0
x = abs(x)
result = ""

if x == 0:
    result = "0"

while x > 0:
    result = str(x % 2) + result
    x //= 2

if sign:
    result = "-" + result

print(f"x --> {result}")
