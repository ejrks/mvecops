const MAXIMUM_REDUCTIONS_DECORNERING: u32 = 10;

use crate::Vmatrix;

// Might not belong here
struct CountingPointer {
    current: usize,
    up: usize,
    down: usize,
}

impl CountingPointer {
    fn all_up(&mut self) {
        self.current += 1;
        self.down += 1;
        self.up += 1;
    }

    fn current(&mut self) -> usize {
        self.current
    }
    fn down(&mut self) -> usize {
        self.down
    }
    fn up(&mut self) -> usize {
        self.up
    }
}
// ------------------------ //

/// Let a set of data have entries with either values or a 0. Accumulation shows the entries around
/// which most of the data accumulates.
///
/// # Premise
///
/// This was made to process an image that has points with alpha=0, and other points with alpha>0.
/// By triming the points that aren't completely surrounded, one can extract through iterations an image
/// that holds less and less data while preserving the shape, until any "stroke" of data is 1px wide and
/// it dissappears.
///
/// "samplekanji.txt" is used as the input sample. Running cargo test will create a file called "accumulations.txt"
/// as a result and "reduction#[number].txt" to show the steps in between.
///
pub fn get_accumulation(input_data: &Vmatrix<u32>, output_path: &Option<&str>) -> Vmatrix<u32>{
    let mut working_data = input_data.clone();

    let mut reductions: u32 = 1;
    let mut process: bool = true;
    let mut accumulative_data: Vec::<Vmatrix<u32>> = Vec::new();

    while process && reductions < MAXIMUM_REDUCTIONS_DECORNERING {
        let new_data: Vmatrix<u32> = decorner_once(&working_data, &mut process);
        // let new_data: Vmatrix<u32> = decorner_once(input_data, &mut process);

        accumulative_data.push(new_data.clone());

        working_data = new_data;

        match output_path {
            None => (),
            Some(target_path) => {
                let path_name: String = 
                    target_path.to_string() +
                    &reductions.to_string() +
                    &String::from(".txt");
                working_data.write_to_file(path_name);
            }
        }

        reductions += 1;
    };

    accumulate_reductions(&accumulative_data)
}

/// Remove any entry in the sample data that is not surrounded by data too.
///
pub fn decorner_once(input_data: &Vmatrix<u32>, two_points_in_row: &mut bool) -> Vmatrix<u32> {
    let mut result: Vmatrix::<u32> = Vmatrix::<u32>::initialize(input_data.size, 0);

    set_bound_rows_to_zero(&mut result);
    *two_points_in_row = process_corners(&input_data, &mut result);

    result
}

/// See [decorner_once]. As data on the first and last entry cannot be completelly surrounded by data,
/// the result data is already set to zero on these rows.
///
pub fn set_bound_rows_to_zero(input_data: &mut Vmatrix<u32>) {
    let input_size: usize = input_data.size;
    let last_entry: usize = input_size * input_size;
    for i in 0..input_size {
        input_data.data[i] = 0;
        input_data.data[last_entry - 1 - i] = 0;
    }
}

/// Goes through the input_data, checking that, if an entry has data, if all the surrounding entries also
/// have data. The result is written to a mutable vector that has to be read separately. The returning value
/// is made to hint [get_accumulation] so it stops processing if no more data will be processed.
///
pub fn process_corners(input_data: &Vmatrix<u32>, output_data: &mut Vmatrix<u32>) -> bool {
    let mut two_points_in_a_row = false;
    let mut previous_active = false;

    let mut pointer_module = 0;

    let row_size = input_data.size;

    let mut pointer = CountingPointer {
        current: row_size + 1,
        up: 0 + 1,
        down: row_size * 2 + 1,
    };

    let last_pointer = (row_size * row_size) - row_size;

    let mut surrounding_all_non_zero = false;

    while pointer.current() != last_pointer {
        pointer_module = pointer.current() % row_size;
        if pointer_module == 0 {
            previous_active = false;

            pointer.all_up();

            continue;
        }
        if pointer_module == (row_size - 1) {
            previous_active = false;

            pointer.all_up();

            continue;
        }

        surrounding_all_non_zero =
            input_data.data[pointer.current()] > 0 &&
            input_data.data[pointer.current() - 1] > 0 &&
            input_data.data[pointer.current() + 1] > 0 &&

            input_data.data[pointer.up()] > 0 &&
            input_data.data[pointer.up() - 1] > 0 &&
            input_data.data[pointer.up() + 1] > 0 &&

            input_data.data[pointer.down()] > 0 &&
            input_data.data[pointer.down() - 1] > 0 &&
            input_data.data[pointer.down() + 1] > 0;

        if surrounding_all_non_zero {
            output_data.data[pointer.current()] = 1;

            if !two_points_in_a_row && previous_active {
                two_points_in_a_row = true;
            }

            previous_active = true;
        } else {
            previous_active = false;
        }

        pointer.all_up();
    }

    two_points_in_a_row
}

/// Sum all the entries of all the vectors within another vector to get the accumulated sum on a single
/// vector.
///
/// # Refactor
///
/// Can be made more general.
///
pub fn accumulate_reductions(input_data: &Vec::<Vmatrix<u32>>) -> Vmatrix<u32> {
    let mut data_length: usize = input_data[0].data.len();

    let mut result: Vmatrix<u32> = Vmatrix::<u32>::initialize(input_data[0].size, 0);
    let mut writing_to = &mut result.data;

    for item in input_data {
        let reading_from = &item.data;
        for x in 0..data_length {
            writing_to[x] = writing_to[x] + reading_from[x];
        }
    }

    result
}

