/// Represent the directions on a plane as the corresponding trigonometric functions. The order chosen
/// follows the derivative of the functions so (cos)'=nsin ; (nsin)'=ncos and so on.
///
#[derive(PartialEq, Debug, Clone)]
pub enum Trigonometric {
    COS = 0,
    NSIN = 1,
    NCOS = 2,
    SIN = 3,
}

impl Trigonometric {
    /// Get trigonometric enum value from an integer
    ///
    /// # Panics
    ///
    /// You have to use a value possible for the enum, this is 0, 1, 2 or 3
    ///
    pub fn from_int(value: usize) -> Trigonometric {
        match value {
            0 => Trigonometric::COS,
            1 => Trigonometric::NSIN,
            2 => Trigonometric::NCOS,
            3 => Trigonometric::SIN,
            other => {
                panic!("Value passed couldn't be converted. Values must be in range [0==3]. Your value was {other}");
            }
        }
    }
    
    /// Get the derivative of a direction
    ///
    pub fn derivative(direction: &Trigonometric) -> Trigonometric {
        let mut int_value = direction.clone() as usize;
        int_value = (int_value + 1) % 4;
        Trigonometric::from_int(int_value)
    }

    /// Get the antiderivative of a direction
    ///
    /// # Panics - should not
    /// 
    /// Unwrap should not fail, as the value is always forced to 0 when it becomes negative
    ///
    pub fn antiderivative(direction: &Trigonometric) -> Trigonometric {
        let mut int_value = direction.clone() as i32;
        int_value -= 1;
        if int_value < 0 {
            int_value = 3;
        }
        Trigonometric::from_int(int_value.try_into().unwrap())
    }

    /// Let a matrix representing a vector have a row_size, given a starting_point index on any entry
    /// the function finds the index when moving in a specific direction. This is, COS will get the 
    /// index on the "right", SIN would get the index that is "above" the current one.
    /// Offset45 applies a 45 degree rotation (positive rotation counterclock-wise) too, so in the last case
    /// SIN with offset < 0, would get the index "above" + "right" to the current one, while a positive offset
    /// would get the index "above" + "left" to the current one.
    ///
    pub fn get_index_from_direction (starting_point: usize, row_size: usize, direction: &Trigonometric, offset45: i32) -> usize {
        let mut result = starting_point;

        match direction {
            Trigonometric::COS =>  result = result + 1,
            Trigonometric::NCOS => result = result - 1,
            Trigonometric::NSIN => result = result + row_size,
            Trigonometric::SIN =>  result = result - row_size,
        }

        if offset45 < 0 {
            let derivative = Trigonometric::derivative(&direction);
            result = Self::get_index_from_direction(result, row_size, &derivative, 0);
        }
        if offset45 > 0 {
            let antiderivative = Trigonometric::antiderivative(&direction);
            result = Self::get_index_from_direction(result, row_size, &antiderivative, 0);
        }

        result
    }
}