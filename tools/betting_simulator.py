import random

pot = 10
promised_premium_yes = 0
promised_premium_no = 0

for i in range(0, 100):
    bet = random.choice([True, False])
    pot += 10
    print("offer premium yes: ", (pot - promised_premium_yes) * 0.8)
    print("offer premium no: ", (pot - promised_premium_no) * 0.8)
    if bet:
        promised_premium_yes += (pot - promised_premium_yes) * 0.8
    else:
        promised_premium_no += (pot - promised_premium_no) * 0.8
print(promised_premium_yes)
print(promised_premium_no)
print(pot)