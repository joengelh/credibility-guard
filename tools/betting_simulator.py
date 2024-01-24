import random

pot = 10
promised_premium_yes = 0
promised_premium_no = 0

for i in range(0, 100):
    bert_direction = random.choice([True, False])
    bet_size = random.randint(1, 100)
    pot += bet_size
    # calculate the bet compared to pot
    bet_weight = bet_size / pot
    print(bet_weight)
    print("bet size: ", bet_size)
    print("offer premium yes: ",
          ((pot - promised_premium_yes) * 0.7 * bet_weight * 0.7) + bet_size)
    print("offer premium no: ",
          ((pot - promised_premium_no) * 0.7 * bet_weight * 0.7) + bet_size)
    if bert_direction:
        promised_premium_yes += ((pot - promised_premium_yes)
                                 * 0.8 * bet_weight) + bet_size
    else:
        promised_premium_no += ((pot - promised_premium_no)
                                * 0.8 * bet_weight) + bet_size
print(promised_premium_yes)
print(promised_premium_no)
print(pot)
