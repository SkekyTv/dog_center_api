use garde::{Error as GardeError, Result as GardeResult};

pub fn create_vec_range_validator<T>(min: T, max: T) -> impl Fn(&Option<Vec<T>>, &()) -> GardeResult
where
    T: PartialOrd + Copy + std::fmt::Display + 'static,
{
    move |numbers: &Option<Vec<T>>, _ctx: &()| {
        if let Some(vec) = numbers {
            for &num in vec {
                if num < min || num > max {
                    return Err(GardeError::new(format!(
                        "Each number must be between {} and {}",
                        min, max
                    )));
                }
            }
        }
        Ok(())
    }
}
