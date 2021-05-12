# Towers of Hanoi example
def toh(n, fr, to, spare):
    if n == 1:
        print(f"Moved {fr} to {to}.")
    else:
        # move smaller stack to spare spike
        toh(n - 1, fr, spare, to)
        # move current target disc to target spike
        toh(1, fr, to, spare)
        # move smaller stack that is on the spare onto the target spike
        toh(n - 1, spare, to, fr)

toh(4, 1, 2, 3)
