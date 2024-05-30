import nutrition_calculator

result = nutrition_calculator.calculate_nutrition(
    "MALE",   # Sex
    100.0,    # Total Weight
    25,       # Body Fat
    1,        # Daily Activity Score
    23,       # Age
    5,        # Stress Level
    "MuscleGain"  # Goal
)
print(result)