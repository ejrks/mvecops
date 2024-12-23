use crate::Vmatrix;
use crate::Trigonometric;

/// If some curve is ignored, you can try to increase this value to make more checks before the
/// process "gives up". Big numbers might throw it into a very consuming and long-lasting loop.
/// For all working tests, the value for best compromise speed/accuracy was 8.
///
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

pub fn get_curves(global_data: &mut GlobalCurveData, input_data: &Vmatrix<u32>) -> Vmatrix<u32> {
    let set_length = input_data.data.len();
    
    let mut result_set = Vmatrix::<u32>::initialize(global_data.row_size, 0);

    let working_input = &input_data.data;
    let working_result = &mut result_set.data;

    for i in 0..set_length {
        if working_input[i] == 1 && working_result[i] == 0 {
            working_result[i] = 2;
            // draw curve
            // hollow set
        } 
    }

    result_set
}

// Index is the last one in the call
pub fn draw_curve_on(input_data: &Vmatrix<u32>, index: usize) {
    // Should have a working result_set by this point - on C# we used to call a "TestOrInitialize"

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

        // index_natural_direction = 
    }
}

pub fn paint_if_natural_direction(from_index: usize, row_size: usize, direction: Trigonometric, offset: usize) {
    let mut new_index = from_index;

    // let result_direction = 
}