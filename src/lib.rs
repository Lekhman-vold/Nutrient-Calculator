mod enums;

use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use crate::enums::*;

struct NutrientCalculator {
    sex: SexEnum,
    total_weight: f32,
    body_fat: u32,
    daily_activity_score: u32,
    age: u32,
    stress_level: u32,
    goal: GoalEnum,
}

impl NutrientCalculator {
    fn new(
        sex: SexEnum,
        total_weight: f32,
        body_fat: u32,
        daily_activity_score: u32,
        age: u32,
        stress_level: u32,
        goal: GoalEnum,
    ) -> Self {
        Self {
            sex,
            total_weight,
            body_fat,
            daily_activity_score,
            age,
            stress_level,
            goal,
        }
    }

    fn calculate_clean_weight(&self) -> f32 {
        match self.sex {
            SexEnum::MALE => self.calculate_male_clean_weight(),
            SexEnum::FEMALE => self.calculate_female_clean_weight(),
        }
    }

    fn calculate_male_clean_weight(&self) -> f32 {
        match self.body_fat {
            0..=11 => self.total_weight * 0.9,
            12..=14 => self.total_weight * 0.865,
            15..=19 => self.total_weight * 0.825,
            20..=24 => self.total_weight * 0.775,
            25..=29 => self.total_weight * 0.725,
            30..=34 => self.total_weight * 0.675,
            _ => self.total_weight * 0.6,
        }
    }

    fn calculate_female_clean_weight(&self) -> f32 {
        match self.body_fat {
            0..=14 => self.total_weight * 0.9,
            15..=18 => self.total_weight * 0.865,
            19..=21 => self.total_weight * 0.825,
            22..=26 => self.total_weight * 0.775,
            27..=31 => self.total_weight * 0.725,
            32..=36 => self.total_weight * 0.675,
            _ => self.total_weight * 0.6,
        }
    }

    fn calculate_calories(&self, clean_weight: f32) -> f32 {
        let mut calories = clean_weight * (
            27.0 + self.daily_activity_score as f32
        );

        // Consider age
        calories *= match self.age {
            0..=24 => 1.0,
            25..=34 => 0.97,
            35..=44 => 0.94,
            45..=54 => 0.91,
            55..=64 => 0.88,
            _ => 0.84,
        };

        // Consider stress level
        calories *= match self.stress_level {
            1 => 1.065,
            2 => 1.13,
            3 => 1.195,
            4 => 1.26,
            _ => 1.325,
        };

        // Consider goal
        calories *= match self.goal {
            GoalEnum::FatLoss => 0.85,
            GoalEnum::Maintain => 1.0,
            GoalEnum::MuscleGain => 1.15,
        };

        calories
    }

    fn calculate_macros(
        &self,
        clean_weight: f32,
        calories: f32,
    ) -> (f32, f32, f32) {
        match self.sex {
            SexEnum::MALE => self.calculate_male_macros(clean_weight, calories),
            SexEnum::FEMALE => self.calculate_female_macros(clean_weight, calories),
        }
    }

    fn calculate_male_macros(
        &self,
        clean_weight: f32,
        calories: f32,
    ) -> (f32, f32, f32) {
        let (protein, fat) = match self.goal {
            GoalEnum::MuscleGain => (clean_weight * 1.5, clean_weight),
            GoalEnum::Maintain => (clean_weight * 2.0, clean_weight * 0.9),
            GoalEnum::FatLoss => (clean_weight * 3.0, clean_weight * 0.8),
        };

        let carbs = (calories - (protein * 4.0 + fat * 9.0)) / 4.0;
        (protein, fat, carbs)
    }

    fn calculate_female_macros(
        &self,
        clean_weight: f32,
        calories: f32,
    ) -> (f32, f32, f32) {
        let (protein, fat) = match self.goal {
            GoalEnum::MuscleGain => (clean_weight, clean_weight * 1.5),
            GoalEnum::Maintain => (clean_weight * 1.5, clean_weight * 1.2),
            GoalEnum::FatLoss => (clean_weight * 2.0, clean_weight),
        };

        let carbs = (calories - (protein * 4.0 + fat * 9.0)) / 4.0;
        (protein, fat, carbs)
    }

    #[no_mangle]
    fn calculate_nutrition(&self) -> (f32, f32, f32, f32, f32) {
        let clean_weight = self.calculate_clean_weight();
        let calories = self.calculate_calories(clean_weight);
        let (protein, fat, carbs) = self.calculate_macros(
            clean_weight,
            calories,
        );
        (clean_weight, calories, protein, fat, carbs)
    }
}

#[pyfunction]
fn calculate_nutrition(
    sex: String,
    total_weight: f32,
    body_fat: u32,
    daily_activity_score: u32,
    age: u32,
    stress_level: u32,
    goal: String,
) -> PyResult<(f32, f32, f32, f32, f32)> {
    // Convert string parameters to corresponding enums
    let sex_enum = match sex.as_str() {
        "MALE" => SexEnum::MALE,
        "FEMALE" => SexEnum::FEMALE,
        _ => return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("Invalid sex")),
    };

    let goal_enum = match goal.as_str() {
        "MuscleGain" => GoalEnum::MuscleGain,
        "FatLoss" => GoalEnum::FatLoss,
        "Maintain" => GoalEnum::Maintain,
        _ => return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("Invalid goal")),
    };

    let calculator = NutrientCalculator::new(
        sex_enum,
        total_weight,
        body_fat,
        daily_activity_score,
        age,
        stress_level,
        goal_enum,
    );

    let result = calculator.calculate_nutrition();
    Ok(result)
}

#[pymodule]
#[pyo3(name = "nutrient_calculator")]
fn nutrient_calculator(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(calculate_nutrition, m)?)?;
    Ok(())
}
