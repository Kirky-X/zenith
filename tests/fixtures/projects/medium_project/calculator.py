class Calculator:
    def __init__(self, precision=2):
        self.precision = precision
        self.history = []

    def add(self, a, b):
        result = a + b
        self.history.append(f"{a} + {b} = {result}")
        return round(result, self.precision)

    def subtract(self, a, b):
        result = a - b
        self.history.append(f"{a} - {b} = {result}")
        return round(result, self.precision)

    def multiply(self, a, b):
        result = a * b
        self.history.append(f"{a} * {b} = {result}")
        return round(result, self.precision)

    def divide(self, a, b):
        if b == 0:
            raise ValueError("Cannot divide by zero")
        result = a / b
        self.history.append(f"{a} / {b} = {result}")
        return round(result, self.precision)

    def clear_history(self):
        self.history.clear()

    def get_history(self):
        return self.history.copy()


if __name__ == "__main__":
    calc = Calculator()
    print(f"2 + 3 = {calc.add(2, 3)}")
    print(f"10 - 4 = {calc.subtract(10, 4)}")
    print(f"5 * 6 = {calc.multiply(5, 6)}")
    print(f"20 / 4 = {calc.divide(20, 4)}")
    print(f"History: {calc.get_history()}")
