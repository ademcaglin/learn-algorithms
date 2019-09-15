def fibonacci(n):
    a, b = 0, 1
    while a < n:
        print(a, end = ' ')
        a, b = b, a+b
    
def divisible_with_compare():
    l = []
    for i in range(0, 320100):
        if (i % 7 == 0) and (i % 5 != 0):
            l.append(str(i))
    return l

def divisible_without_compare():
    l = []
    for i in range(0, 32010, 7):
        if (i % 5 != 0):
            l.append(str(i))
    return l


