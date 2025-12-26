class BankAccount:
    def __init__(self, owner, balance=0):
        self.owner = owner
        self._balance = balance

    def deposit(self, amount):
        if amount > 0:
            self._balance += amount
            return True
        return False

    def withdraw(self, amount):
        if amount > 0 and amount <= self._balance:
            self._balance -= amount
            return True
        return False

    def get_balance(self):
        return self._balance

if __name__ == "__main__":
    account = BankAccount("Alice", 100)
    account.deposit(50)
    account.withdraw(30)
    print(f"Balance: {account.get_balance()}")
