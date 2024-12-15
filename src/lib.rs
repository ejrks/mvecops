use std::error::Error;

use std::vec::Vec;

use std::fs;

// OUT //
trait Number {}
impl Number for u32 {}

const MAXIMUM_REDUCTIONS_DECORNERING: u32 = 10;

fn textfile_to_int_vector(file_path: String) -> Result<Vec<u32>, Box<dyn Error>> {
    let file_content = fs::read_to_string(file_path)?;
    let mut all_data: Vec<u32> = Vec::new();
    for character in file_content.chars() {
        match character.to_digit(10) {
            Some(data) => all_data.push(data),
            None => continue,
        }
    }

    Ok(all_data)
}

struct Vmatrix<T>
where
    T: Number,
{
    size: usize,
    data: Vec<T>,
}

impl<T> Vmatrix<T> 
where
    T: Number,
{
    fn build(size: usize, new_data: Vec<T>) -> Vmatrix<T> {
        let mut new_instance = Vmatrix {
            data: Vec::new(),
            size,
        };

        for entry in new_data {
             new_instance.data.push(entry);
        }

        new_instance
    }

    fn new(size: usize) -> Vmatrix<T> {
        Vmatrix {
            data: Vec::new(),
            size,
        }
    }
}

fn textfile_to_vmatrix(file_path: String, size: usize) -> Vmatrix<u32> 
{
    match textfile_to_int_vector(file_path) {
        Err(error) => panic!("Input data couldn't be retrieved: {}", error),
        Ok(all_data) => {
            Vmatrix::<u32>::build(size, all_data)
        }
    }
}

fn set_bound_rows_to_zero(input_data: &mut Vmatrix<u32>) {
    let input_size: usize = input_data.size;
    let last_entry: usize = input_size * input_size;
    for i in 0..input_size {
        input_data.data[i] = 0;
        input_data.data[last_entry - 1 - i] = 0;
    }
}

fn process_corners(input_data: &Vmatrix<u32>, output_data: &mut Vmatrix<u32>) -> bool{
    let mut two_points_in_a_row = false;
    let mut previous_active = false;

    let mut pointer_module = 0;

    let row_size = input_data.size;

    let mut current_pointer = row_size + 1;
    let mut up_pointer = 0 + 1;
    let mut dw_pointer = row_size * 2 + 1;
    let last_pointer = (row_size * row_size) - row_size;

    let mut all_pointers_up = |mut x, mut y, mut z| {
        x += 1;
        y += 1;
        z += 1;
    };

    let mut surrounding_all_non_zero = false;

    while current_pointer != last_pointer {
        pointer_module = current_pointer % row_size;
        if pointer_module == 0 {
            previous_active = false;
            all_pointers_up(current_pointer, up_pointer, dw_pointer);
            continue;
        }
        if pointer_module == (row_size - 1) {
            previous_active == false;
            all_pointers_up(current_pointer, up_pointer, dw_pointer);
            continue;
        }

        surrounding_all_non_zero =
            input_data.data[current_pointer] > 0 &&
            input_data.data[current_pointer - 1] > 0 &&
            input_data.data[current_pointer + 1] > 0 &&

            input_data.data[up_pointer] > 0 &&
            input_data.data[up_pointer - 1] > 0 &&
            input_data.data[up_pointer + 1] > 0 &&

            input_data.data[dw_pointer] > 0 &&
            input_data.data[dw_pointer - 1] > 0 &&
            input_data.data[dw_pointer + 1] > 0;

        if surrounding_all_non_zero {
            output_data.data[current_pointer] = 1;

            if !two_points_in_a_row && previous_active {
                two_points_in_a_row = true;
            }

            previous_active = true;
        } else {
            previous_active = false;
        }

        all_pointers_up(current_pointer, up_pointer, dw_pointer);
    }

    two_points_in_a_row
}

fn decorner_once(input_data: &Vmatrix<u32>, two_points_in_row: &mut bool) -> Vmatrix<u32> {
    // This might be wrong, initialization should be on zero
    let mut result: Vmatrix::<u32> = Vmatrix::<u32>::build(input_data.size, input_data.data.clone());

    set_bound_rows_to_zero(&mut result);
    *two_points_in_row = process_corners(&input_data, &mut result);

    result
}

fn get_accumulation(input_data: &Vmatrix<u32>, write_out: Option<bool>) -> Vmatrix<u32> {
    let writing_out: bool = write_out.unwrap_or(false);

    let mut reductions: u32 = 1;
    let mut process: bool = true;
    let accumulativeData: Vec::<Vmatrix<u32>>;

    while process && reductions < MAXIMUM_REDUCTIONS_DECORNERING {
        let new_data: Vmatrix<u32> = decorner_once(&input_data, &mut process);
    };

    Vmatrix::<u32>::new(0)
}

// --- //

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_get_sample_data() {
        let file_content = fs::read_to_string("samplekanji.txt");
        for content in file_content {
            println!("{}", content);
        }
    }

    #[test]
    fn can_get_input_from_sample() {
        match textfile_to_int_vector(String::from("samplekanji.txt")) {
            Ok(all_data) => assert_eq!(all_data.len(), 64 * 64),
            Err(error) => panic!("Input data couldn't be retrieved: {}", error),
        }
    }

    #[test]
    fn can_generate_sample_struct() {
        let sample_size = 64;

        let sample_data: Vmatrix<u32> = textfile_to_vmatrix(String::from("samplekanji.txt"), sample_size);
        assert_eq!(sample_data.size, sample_size);
        assert_eq!(sample_data.data.len(), sample_size * sample_size);
    }

    #[test]
    fn bound_rows_zero_working() {
        let sample_data_3 = vec![1, 0, 1, 1, 1, 0, 0, 0, 1];
        let sample_comp_3 = vec![0, 0, 0, 1, 1, 0, 0, 0, 0];
        let sample_data_4 = vec![1, 0, 1, 0, 1, 1, 0, 1, 1, 0, 1, 1, 0, 0, 1, 1];
        let sample_comp_4 = vec![0, 0, 0, 0, 1, 1, 0, 1, 1, 0, 1, 1, 0, 0, 0, 0];
        let sample_data_5 = vec![1, 0, 1, 0, 1, 1, 0, 1, 1, 0, 1, 1, 0, 0, 1, 1, 1, 0, 1, 1, 1, 0, 0, 0, 1];
        let sample_comp_5 = vec![0, 0, 0, 0, 0, 1, 0, 1, 1, 0, 1, 1, 0, 0, 1, 1, 1, 0, 1, 1, 0, 0, 0, 0, 0];

        let mut data_sample = Vmatrix {
            size: 3,
            data: sample_data_3,
        };
        set_bound_rows_to_zero(&mut data_sample);
        assert_eq!(data_sample.data, sample_comp_3); 

        data_sample.size = 4;
        data_sample.data = sample_data_4;
        set_bound_rows_to_zero(&mut data_sample);
        assert_eq!(data_sample.data, sample_comp_4); 

        data_sample.size = 5;
        data_sample.data = sample_data_5;
        set_bound_rows_to_zero(&mut data_sample);
        assert_eq!(data_sample.data, sample_comp_5); 
    }
}
