from enum import Enum
from nutrition_calculator import calculate_nutrition


class SexEnum(Enum):
    MALE = 'MALE'
    FEMALE = 'FEMALE'


class GoalEnum(Enum):
    MUSCLE_GAIN = 'MuscleGain'
    FAT_LOSS = 'FatLoss'
    MAINTAIN = 'Maintain'


result = calculate_nutrition(
    SexEnum.MALE.value,  # Sex
    100.0,  # Total Weight
    25,  # Body Fat
    1,  # Daily Activity Score
    23,  # Age
    5,  # Stress Level
    GoalEnum.MUSCLE_GAIN.value  # Goal
)
print(result)
