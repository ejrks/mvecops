use crate::Vmatrix;
use crate::Trigonometric;
use crate::Vector2;

use crate::get_index_distance;
use crate::get_middle_point;

use crate::row_distance;

use std::vec::Vec;

/// If some curve is ignored, you can try to increase this value to make more checks before the
/// process "gives up". Big numbers might throw it into a very consuming and long-lasting loop.
/// For all working tests, the value for best compromise accuracy/safety was 8.
///
/// # Testing
///
/// If your code starts panicking, your first approach shouldn't be changing this value. When
/// [draw_curve_on] panics because it reached its maximum number of checks it usually means that
/// it is reading data incorrectly.
/// During testing, for instance, the calls to [hollow_set] wouldn't clean up the inside of the
/// already processed curves, which made the method paint curves within curves. This value is
/// mostly to find runtime bugs and prevent infinite loops, not to improve performance.
pub const MAX_CHECKS_FACTOR: usize = 8;

/// Global shared data for a series of closed curve operations
///
pub struct GlobalCurveData {
    /// All operations share row size
    ///
    pub row_size: usize,
    
    /// All the points that define separte curves
    ///
    pub curves_global_output: Vmatrix<u32>,
    /// The order of the points that define each curve
    ///
    pub curves_global_orderd: Vmatrix<u32>,

    /// Each new curve is defined by this increasing value
    ///
    pub global_output_number: usize,
    /// Each new point within the current curve is ordered according to this increasing value
    ///
    pub global_orderd_cardin: usize,
}

impl GlobalCurveData {
    /// Set the initial values for a GlobalCurveData that will be shared throughout the operation
    ///
    pub fn new(size: usize) -> GlobalCurveData {
        GlobalCurveData {
            row_size: size,

            curves_global_output: Vmatrix::<u32>::initialize(size, 0),
            curves_global_orderd: Vmatrix::<u32>::initialize(size, 0),

            global_output_number: 1,
            global_orderd_cardin: 0,
        }
    }

    /// Transpose both internal matrices for this instance of GlobalCurveData, allowing to operate
    /// against fixed data that couldn't be transposed
    ///
    pub fn transpose_internal(&mut self) {
        self.curves_global_output.transpose();
        self.curves_global_orderd.transpose();
    }
}

/// On a data set, find all the closed bodies that represent a curve, when the vector is represented
/// as a matrix.
///
/// # Testing
///
/// This method has been tested against datasets that were treated first to separate the parts of the
/// data where there were long rows on one hand, and long columns on the other hand, basically removing
/// any lines except those that had a slope != 0
pub fn get_curves(global_data: &mut GlobalCurveData, input_data: &Vmatrix<u32>) -> Vmatrix<u32> {
    let set_length = input_data.data.len();
    
    let mut result_set = Vmatrix::<u32>::initialize(global_data.row_size, 0);

    let working_input = &input_data.data;

    let mut curve_count = 0;

    for i in 0..set_length {
        if working_input[i] == 1 && result_set.data[i] == 0 {
            result_set.data[i] = 2;
            draw_curve_on(&input_data, &mut result_set, &global_data, i);
            hollow_set(2, 1, global_data.row_size, &input_data, &mut result_set);
            curve_count += 1;
        } 
    }

    result_set
}

/// Mark values that have been processed already two prevent drawing the curves out of them on loop
///
fn hollow_set(anchor_value: u32, hollow_value: u32, row_size: usize, input_data: &Vmatrix<u32>, result_set: &mut Vmatrix<u32>) {
    let set_size = result_set.data.len();
    println!("Size of the set {}", set_size);

    let mut row_count = 0;
    let mut anchor_enabled: bool = false;

    let working_input = &input_data.data;
    let working_result = &mut result_set.data;

    for i in 0..set_size {
        if anchor_enabled && (working_input[i] != 0) && (working_result[i] != anchor_value) {
            working_result[i] = hollow_value;
        }
        if anchor_enabled && working_input[i] == 0 {
            anchor_enabled = !anchor_enabled;
        }
        if working_result[i] == anchor_value {
            anchor_enabled = true;
        }

        row_count += 1;
        if row_count >= row_size {
            anchor_enabled = false;
            row_count = 0;
        }
    }
}

/// From the input_data, paint on the result_output only the outline of the different closed bodies found within
/// when input_data's internal vector is represented as a matrix of a certain row_size, passed through the global
/// data object.
///
/// # Example
///
/// When running cargo test, the output file "inflexion.txt" shows an example of what the output would be for a
/// data like that provided by "samplekanji.txt". You should see any line that isn't completely straigth, surrounded
/// by "2", and with "1" in the interior. Also, the resulting curves will take up less space than the original.
/// Straight lines are treated separately.
fn draw_curve_on(input_data: &Vmatrix<u32>, result_output: &mut Vmatrix<u32>, global_data: &GlobalCurveData, index: usize) {
    let mut current_direction = Trigonometric::COS;
    let mut current_index = index;
    let mut set_length = input_data.data.len();
    let mut number_of_checks = 0;
    let maximum_checks = set_length * MAX_CHECKS_FACTOR;

    let mut index_natural_direction: usize;
    let mut index_45_degrees: usize;
    let mut index_overdue_direction: usize;

    let mut cardinal_changes: usize = 0;
    let mut last_index_in_loop: i32 = -1;

    let mut index_of_best_distance: i32 = -1;
    let mut current_index_distance = 0.0;
    let mut current_distance_bestv = 0.0;

    while current_index < set_length && number_of_checks < maximum_checks {
        if cardinal_changes >= 4 && last_index_in_loop == current_index as i32 {
            break;
        }

        last_index_in_loop = current_index as i32;

        index_natural_direction = paint_on_direction(current_index, global_data.row_size, &current_direction, 0, input_data, result_output);
        if index_natural_direction != current_index {
            current_index = index_natural_direction;
            cardinal_changes = 0;
            
            number_of_checks += 1;

            if index_natural_direction == index {
                return;
            }

            continue;
        }

        index_45_degrees = paint_on_direction(current_index, global_data.row_size, &current_direction, -1, input_data, result_output);
        if index_45_degrees != current_index {
            current_index = index_45_degrees;
            cardinal_changes = 0;

            number_of_checks += 1;

            if index_45_degrees == index {
                return;
            }

            continue;
        }

        index_overdue_direction = paint_on_direction(current_index, global_data.row_size, &Trigonometric::derivative(&current_direction), 0, input_data, result_output);
        if index_overdue_direction != current_index {
            current_index = index_overdue_direction;
            cardinal_changes = 0;

            number_of_checks += 1;

            if index_overdue_direction == index {
                return;
            }

            continue;
        }

        current_direction = Trigonometric::derivative(&current_direction);
        cardinal_changes += 1;

        number_of_checks += 1;
        if number_of_checks >= maximum_checks {
            result_output.data[index] = 6;
            result_output.write_to_file(String::from("debug.txt"));
            panic!("The process gave up before checking all values. Is MAX_CHECKS_FACTOR too low? Index calling: {}", index)
        }
    }
}

fn paint_on_direction(from_index: usize, row_size: usize, direction: &Trigonometric, offset: i32, input_data: &Vmatrix<u32>, result_output: &mut Vmatrix<u32>) -> usize {
    let mut new_index = from_index;

    let result_direction = Trigonometric::get_index_from_direction(from_index, row_size, &direction, offset);
    if input_data.test_index(result_direction) {
        if input_data.data[result_direction] == 1 && result_output.data[result_direction] != 1 {
            result_output.data[result_direction] = 2;
            new_index = result_direction;
        }
    }

    return new_index
}

fn get_smooth_curves(input_data: &Vmatrix<u32>, from_point: usize, to_point: usize, row_size: usize) -> Vec<usize> {
    let mut result_list: Vec<usize> = vec![];

    let delta1: &mut Vector2<i32> = &mut Vector2::new(0, 0);
    let delta2: &mut Vector2<i32> = &mut Vector2::new(0, 0);

    let middle_point = get_middle_point(from_point as i32, to_point as i32, row_size, delta1, delta2);
    if middle_point == from_point || middle_point == to_point {
        return result_list;
    }

    let working_data = &input_data.data;

    // backup was originally used here
    if working_data[middle_point] == 1 {
        result_list.push(middle_point);
    }
    else {
        let orthogonal1: &mut Vector2<i32> = &mut Vector2::new(0, 0);
        let orthogonal2: &mut Vector2<i32> = &mut Vector2::new(0, 0);

        //
    }

    return result_list;
}

// Result set is using PREVIOUS CURVE DATA
pub fn mark_curve_points(input_data: &Vmatrix<u32>, result_set: &mut Vmatrix<u32>, global_data: &mut GlobalCurveData, dominant_curve: bool) {
    // We used to make a backup of the input here on C#, but it might be unnecesarry, if input is modified
    // throughout this, create the backup

    let row_size = global_data.row_size;

    let working_input = &input_data.data;

    let mut returning_index: usize = 0;

    let set_length = input_data.data.len();
    for i in 0..set_length {
        global_data.global_orderd_cardin = 0;

        if working_input[i] == 2 && result_set.data[i] == 0 {
            result_set.data[i] = 3;
            returning_index = find_curve_on(&input_data, result_set, global_data, i);

            if row_distance(i as i32, returning_index as i32, row_size) == 0 && !dominant_curve {
                for x in i..=returning_index {
                    result_set.data[x] = 1;
                }

                continue;
            }

            if returning_index == i {
                result_set.data[i] = 1;
            }
            else {
                //
            }
        }
    }
}

fn get_if_curve_value (input_data: &Vmatrix<u32>, result_output: &mut Vmatrix<u32>, from_index: usize, global_data: &GlobalCurveData, direction: &Trigonometric, offset: i32) -> usize {
    let mut new_index = from_index;
    let row_size = global_data.row_size;

    let result_direction = Trigonometric::get_index_from_direction(from_index, row_size, &direction, offset);
    if input_data.test_index(result_direction) {
        if input_data.data[result_direction] == 2 {
            if result_output.data[result_direction] != 3 {
                result_output.data[result_direction] = 1;
            }
            new_index = result_direction;
        }
    }

    return new_index;
}

// Check if the return value is ever used
// ??? result_output was passed as a mutable reference, but then goes directly into get_if_curve_value ??
// Check if the data even changes, cause this can be non-intentional
pub fn find_curve_on(input_data: &Vmatrix<u32>, result_output: &mut Vmatrix<u32>, global_data: &GlobalCurveData, index: usize) -> usize{
    // Should have a working result_set by this point - on C# we used to call a "TestOrInitialize"

    let row_size = global_data.row_size;

    let mut current_direction = Trigonometric::COS;
    let mut current_index = index;
    let mut set_length = input_data.data.len();
    let mut number_of_checks = 0;
    let maximum_checks = set_length * MAX_CHECKS_FACTOR;

    let mut index_natural_direction: usize;
    let mut index_45_degrees: usize;
    let mut index_overdue_direction: usize;

    let mut cardinal_changes: usize = 0;
    let mut last_index_in_loop: i32 = -1;

    let mut index_of_best_distance: usize = 0;
    let mut current_index_distance = 0.0;
    let mut current_distance_best = 0.0;

    while current_index < set_length && number_of_checks < maximum_checks {
        if cardinal_changes >= 4 && last_index_in_loop == current_index as i32 {
            break;
        }

        last_index_in_loop = current_index as i32;

        index_natural_direction = get_if_curve_value(&input_data, result_output, current_index, global_data, &current_direction, 0);
        if index_natural_direction != current_index {
            current_index = index_natural_direction;
            cardinal_changes = 0;
            
            number_of_checks += 1;

            current_index_distance = get_index_distance(index as i32, current_index as i32, row_size);
            if current_index_distance > current_distance_best {
                current_distance_best = current_index_distance;
                index_of_best_distance = current_index;
            }

            if index_natural_direction == index {
                result_output.data[index_of_best_distance] = 3;
                return index_of_best_distance;
            }

            continue;
        }

        index_45_degrees = get_if_curve_value(&input_data, result_output, current_index, global_data, &current_direction, -1);
        if index_45_degrees != current_index {
            current_index = index_45_degrees;
            cardinal_changes = 0;
            
            number_of_checks += 1;

            current_index_distance = get_index_distance(index as i32, current_index as i32, row_size);
            if current_index_distance > current_distance_best {
                current_distance_best = current_index_distance;
                index_of_best_distance = current_index;
            }

            if index_45_degrees == index {
                result_output.data[index_of_best_distance] = 3;
                return index_of_best_distance;
            }

            continue;
        }

        index_overdue_direction = get_if_curve_value(&input_data, result_output, current_index, global_data, &Trigonometric::derivative(&current_direction), 0);
        if index_overdue_direction != current_index {
            current_index = index_overdue_direction;
            cardinal_changes = 0;
            
            number_of_checks += 1;

            current_index_distance = get_index_distance(index as i32, current_index as i32, row_size);
            if current_index_distance > current_distance_best {
                current_distance_best = current_index_distance;
                index_of_best_distance = current_index;
            }

            if index_overdue_direction == index {
                result_output.data[index_of_best_distance] = 3;
                return index_of_best_distance;
            }

            continue;
        }

        current_direction = Trigonometric::derivative(&current_direction);
        cardinal_changes += 1;

        number_of_checks += 1;
        if number_of_checks >= maximum_checks {
            panic!("The process gave up before checking all values. Is MAX_CHECKS_FACTOR too low? Index calling: {}", index)
        }
    }

    // In an old version we returned index again, which should be unchanged, that didn't make sense (?)
    return current_index;
}