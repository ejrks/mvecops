use std::fmt::Debug;

// There is definetely a vector class on crates, don't clutter

/// A struct to save a simple 2d vector
///
#[derive(Copy, Clone)]
pub struct Vector2<T> 
where
    T: Debug,
{
    pub x: T,
    pub y: T,
}

impl<T> Vector2<T> 
where
    T: Debug + Copy + std::cmp::PartialEq,
{
    /// Create a new vector with the given values
    ///
    pub fn new(x: T, y: T) -> Vector2<T> {
        Vector2 {
            x,
            y,
        }
    }

    /// Check if the vectors have the same content
    ///
    pub fn equals(&self, other: &Vector2<T>) -> bool {
        return self.x == other.x && self.y == other.y;
    }
}

/// Returns the sum of two integer vectors
///
pub fn sum_vectors(input1: &Vector2<i32>, input2: &Vector2<i32>) -> Vector2<i32> {
    Vector2 {
            x: input1.x + input2.x,
            y: input1.y + input2.y,
    }
}

/// See [sum_vectors].
///
pub fn sum_i64_vectors(input1: &Vector2<i64>, input2: &Vector2<i64>) -> Vector2<i64> {
    Vector2 {
            x: input1.x + input2.x,
            y: input1.y + input2.y,
    }
}

/// Returns the substraction of two i64 vectors
///
pub fn sub_vectors(input1: &Vector2<i64>, input2: &Vector2<i64>) -> Vector2<i64> {
    Vector2 {
            x: input1.x - input2.x,
            y: input1.y - input2.y,
    }
}

/// Returns the approximate angle between the two vector.
///
/// # Non-mathematical
///
/// When any of the vectors is (0,0) you end up dividing by 0 when trying to get their magnitudes,
/// which would return NaN. As this method is used mainly to check how similar vectors are, a value
/// of -2 is returned instead (which is impossible, as no value of cosine will ever be bigger than /// 1) or 0 if both vectors happen to be the zero vector, which under the intended use means that
/// they are "the same" and there is no cosine distance between them.
///
pub fn cos_between(input1: &Vector2<i64>, input2: &Vector2<i64>) -> f64 {
    let x1 = input1.x as f64;
    let x2 = input2.x as f64;
    let y1 = input1.y as f64;
    let y2 = input2.y as f64;

    if (x1 == 0.0 && y1 == 0.0) || (x2 == 0.0 && y2 == 0.0) {
        if (x1 == 0.0 && y1 == 0.0 && x2 == 0.0 && y2 == 0.0) {
            return 1.0;
        }
        else {
            return -2.0;
        }
    }

    let dot_product = x1 * x2 + y1 * y2;
    let magnitude_1 = (x1 * x1 + y1 * y1).sqrt();
    let magnitude_2 = (x2 * x2 + y2 * y2).sqrt();

    return (dot_product / (magnitude_1 * magnitude_2));
}

/// Returns a scaled vector from the input
///
pub fn scale_vector(input: &Vector2<i64>, scale_factor: i64) -> Vector2<i64> {
    let mut new_x: f64 = input.x as f64 / scale_factor as f64;
    let mut new_y: f64 = input.y as f64 / scale_factor as f64;
    new_x = new_x.floor();
    new_y = new_y.floor();

    Vector2::new(new_x as i64, new_y as i64)
}

/// Get the distance between two indexes within a matrix.
///
pub fn get_index_distance(from: i32, to: i32, row_size: usize) -> f32 {
    let internal_rs = row_size as i32;

    let mut xdistance = column_distance(from, to, row_size) as f32;
    xdistance = xdistance * xdistance;
    let mut ydistance = row_distance(from, to, row_size) as f32;
    ydistance = ydistance * ydistance;
    return (xdistance + ydistance).sqrt();
}

/// Get the index that is in the middle of the line that connects two entries in a matrix.
///
pub fn get_middle_point(from: i32, to: i32, row_size: usize, delta1: &mut Vector2<i32>, delta2: &mut Vector2<i32>) -> usize {
    let internal_rs = row_size as i32;

    let x1 = from % internal_rs;
    let x2 = to % internal_rs;

    let y1 = from / internal_rs;
    let y2 = to / internal_rs;

    let mut x_plus_delta = 0;
    let x_delta = (x2 - x1) / 2;
    if x_delta > 0 {
        x_plus_delta = x1 + x_delta;
    }
    else {
        x_plus_delta = x2 - x_delta;
    }

    let mut y_plus_delta = 0;
    let y_delta = (y2 - y1) / 2;
    if y_delta > 0 {
        y_plus_delta = y1 + y_delta;
    }
    else {
        y_plus_delta = y2 - y_delta;
    }

    *delta1 = Vector2::new(x_delta, y_delta);
    *delta2 = Vector2::new(-x_delta, -y_delta);

    return (x_plus_delta + internal_rs * y_plus_delta).try_into().unwrap();
}

/// Get how many rows separate two indexes within a matrix
///
pub fn row_distance(from: i32, to: i32, row_size: usize) -> i32 {
    let internal_rs = row_size as i32;

    let y1 = from / internal_rs;
    let y2 = to / internal_rs;

    return y2 - y1; 
}

/// Get how many columns separate two indexes within a matrix.
///
pub fn column_distance(from: i32, to: i32, row_size: usize) -> i32 {
    let internal_rs = row_size as i32;

    let x1 = from % internal_rs;
    let x2 = to % internal_rs;

    return x2 - x1; 
}

/// Return an index as a displacement vector starting at the beginning of the vector
///
pub fn get_index_as_coordinates(input: usize, row_size: usize) -> Vector2<i32> {
    let internal_input = input as i32;
    let internal_rs = row_size as i32;

    let x = internal_input % internal_rs;
    let y = internal_input / internal_rs;

    Vector2 {
        x,
        y,
    }
}

/// See [get_index_as_coodrinates].
///
pub fn get_coordinates_from(index: i64, row_size: i64) -> Vector2<i64> {
    let x = index % row_size;
    let y = index / row_size;

    Vector2 {
        x,
        y,
    }
}

/// Check if a test value is "close enough" to another.
///
/// # Non-inclusive
///
/// Let test_value be 0.1, and close_to be 0, if by_margin is 0.1, it returns FALSE.
///
pub fn close_enough(test_value: f32, close_to: f32, by_margin: f32) -> bool {
    if test_value == close_to {
        return true;
    }

    let mut absolute_margin = by_margin;
    if by_margin < 0.0 {
        absolute_margin *= -1.0;
    }  

    let lower_margin = close_to - absolute_margin;
    let upper_margin = close_to + absolute_margin;

    return lower_margin < test_value && test_value < upper_margin;
}

/// See [close_enough].
///
pub fn close_enough_f64(test_value: f64, close_to: f64, by_margin: f64) -> bool {
    if test_value == close_to {
        return true;
    }

    let mut absolute_margin = by_margin;
    if by_margin < 0.0 {
        absolute_margin *= -1.0;
    }  

    let lower_margin = close_to - absolute_margin;
    let upper_margin = close_to + absolute_margin;

    return lower_margin < test_value && test_value < upper_margin;
}

/// Set two vectors to be perpendicular to another couple vectors that are antiparallel
///
pub fn orthogonal_from_antiparallel(input1: &Vector2<i32>, input2: &Vector2<i32>, orthogonal1: &mut Vector2<i32>, orthogonal2: &mut Vector2<i32>) {
    if (input1.x * -1 != input2.x) || (input1.y * -1 != input2.y) {
        panic!("The provided vector aren't antiparallel, they don't fulfill the condition that their
                coordinates have the same absolute values with reversed signs");
    } 

    orthogonal1.x = input2.y;
    orthogonal1.y = input1.x;

    orthogonal2.x = input1.y;
    orthogonal2.y = input2.x;
}

/// Move an index by a vector displacement on the data
///
pub fn array_position_vector_displacement(input_index: usize, row_size: usize, direction: &Vector2<i32>) -> i32 {
    let internal_rs = row_size as i32;

    let mut new_position: Vector2<i32> = get_index_as_coordinates(input_index, row_size);
    new_position = sum_vectors(&new_position, direction);

    let result = new_position.x + internal_rs * new_position.y;
    return result;
}