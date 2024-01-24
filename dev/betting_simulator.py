import random

pot = 33
promised_premium_yes = 0
promised_premium_no = 0

for i in range(0, 100):
    bert_direction = random.choices(
        [True, False], weights=[0.95, 1 - 0.95])[0]
    bet_size = random.randint(1, 100)
    bet_weight = bet_size / pot
    print(bet_size)
    pot += bet_size
    offered_premium_yes = (((pot - promised_premium_yes)
                            ) * bet_weight * 0.95) + bet_size
    offered_premium_no = (((pot - promised_premium_no))
                          * bet_weight * 0.95) + bet_size
    print("offer premium yes: ", offered_premium_yes)
    print("offer premium no: ", offered_premium_no)
    if bert_direction:
        promised_premium_yes += offered_premium_yes
    else:
        promised_premium_no += offered_premium_no
print(promised_premium_yes)
print(promised_premium_no)
print(pot)
