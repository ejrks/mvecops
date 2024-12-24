/// Get the distance between two indexes within a matrix
///
pub fn get_index_distance(from: i32, to: i32, row_size: usize) -> f32 {
    let internal_rs = row_size as i32;

    let x1 = from % internal_rs;
    let x2 = to % internal_rs;

    let y1 = from / internal_rs;
    let y2 = to / internal_rs;

    let mut xdistance = (x2 - x1) as f32;
    xdistance = xdistance * xdistance;
    let mut ydistance = (y2 - y1) as f32;
    ydistance = ydistance * ydistance;
    return (xdistance + ydistance).sqrt();
}

/// Check if a test value is "close enough" to another.
///
/// # Non-inclusive
///
/// Let test_value be 0.1, and close_to be 0, if by_margin is 0.1, it returns FALSE
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