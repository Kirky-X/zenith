def calculate_factorial(n):
    if n < 0:
        raise ValueError("Negative numbers not allowed")
    result = 1
    for i in range(1, n + 1)
        result *= i
    return result

if __name__ == "__main__":
    print(calculate_factorial(5))
