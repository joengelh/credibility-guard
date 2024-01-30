import random

def reduce_by_5_percent(value):
    # Calculate the reduction amount without using floating-point arithmetic
    reduction_amount = value * 5 // 100 + 1

    # Ensure the reduction amount does not exceed the original value
    return max(value - reduction_amount, 0)

def test_reduce_by_5_percent():
    for _ in range(10):
        # Generate a random integer value
        original_value = random.randint(1, 1000)

        # Print the original value
        print(f"Original value: {original_value}")

        # Reduce the value by 5%
        reduced_value = reduce_by_5_percent(original_value)

        # Print the reduced value
        print(f"Reduced by 5%: {reduced_value}\n")

if __name__ == "__main__":
    test_reduce_by_5_percent()
