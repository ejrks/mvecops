use std::fmt::Debug;

/// A struct to save a simple 2d vector
///
pub struct Vector2<T> 
where
    T: Debug,
{
    pub x: T,
    pub y: T,
}

impl<T> Vector2<T> 
where
    T: Debug + Copy,
{
    /// Create a new vector with the given values
    ///
    pub fn new(x: T, y: T) -> Vector2<T> {
        Vector2 {
            x,
            y,
        }
    }
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