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

#[derive(Clone)]
struct Vmatrix<T>
where
    T: Number + ToString + Clone,
{
    size: usize,
    data: Vec<T>,
}


// Initialize and build should be the same method
impl<T> Vmatrix<T> 
where
    T: Number + ToString + Clone,
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

    fn initialize(size: usize, initial_value: T) -> Vmatrix<T> {
        let mut new_instance = Vmatrix {
            data: Vec::new(),
            size,
        };

        let total_data_size = size * size;
        for x in 0..total_data_size {
             new_instance.data.push(initial_value.clone());
        }

        new_instance
    }

    fn new(size: usize) -> Vmatrix<T> {
        Vmatrix {
            data: Vec::new(),
            size,
        }
    }

    fn write_to_file(&self, file_path: String) {
        let mut complete_output: String = String::from("");
        let size = self.size;

        for i in 0..size {
            for j in 0..size {
                complete_output += &self.data[j + i * size].to_string();
            }    
            complete_output += &String::from("\n");
        }
        fs::write(file_path, complete_output);
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

// Porperly factor out pointer counting
fn process_corners(input_data: &Vmatrix<u32>, output_data: &mut Vmatrix<u32>) -> bool {
    let mut two_points_in_a_row = false;
    let mut previous_active = false;

    let mut pointer_module = 0;

    let row_size = input_data.size;

    let mut current_pointer = row_size + 1;
    let mut up_pointer = 0 + 1;
    let mut dw_pointer = row_size * 2 + 1;
    let last_pointer = (row_size * row_size) - row_size;

    let mut surrounding_all_non_zero = false;

    let mut written_times = 0;

    while current_pointer != last_pointer {
        if written_times >= 500 && written_times <= 510 {
                output_data.write_to_file(String::from("dataat5xx.txt"));
        }

        pointer_module = current_pointer % row_size;
        if pointer_module == 0 {
            previous_active = false;
            
            current_pointer += 1;
            up_pointer += 1;
            dw_pointer += 1;

            continue;
        }
        if pointer_module == (row_size - 1) {
            previous_active = false;

            current_pointer += 1;
            up_pointer += 1;
            dw_pointer += 1;

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

        current_pointer += 1;
        up_pointer += 1;
        dw_pointer += 1;
    }

    two_points_in_a_row
}

fn decorner_once(input_data: &Vmatrix<u32>, two_points_in_row: &mut bool) -> Vmatrix<u32> {
    // This might be wrong, initialization should be on zero
    let mut result: Vmatrix::<u32> = Vmatrix::<u32>::initialize(input_data.size, 0);

    set_bound_rows_to_zero(&mut result);
    *two_points_in_row = process_corners(&input_data, &mut result);

    result
}

// Data that is gonna be access should hold a reference instead of fetching it every iteration
fn accumulate_reductions(input_data: &Vec::<Vmatrix<u32>>) -> Vmatrix<u32> {
    let mut data_length: usize = input_data[0].data.len();

    let mut result: Vmatrix<u32> = Vmatrix::<u32>::initialize(input_data[0].size, 0);

    for item in input_data {
        for x in 0..data_length {
            result.data[x] = result.data[x] + item.data[x];
        }
    }

    result
}

// Refactor to choose output files
fn get_accumulation(input_data: Vmatrix<u32>, write_out: bool) -> Vmatrix<u32>{
    let mut working_data = input_data.clone();

    let mut reductions: u32 = 1;
    let mut process: bool = true;
    let mut accumulative_data: Vec::<Vmatrix<u32>> = Vec::new();

    while process && reductions < MAXIMUM_REDUCTIONS_DECORNERING {
        let new_data: Vmatrix<u32> = decorner_once(&working_data, &mut process);

        accumulative_data.push(new_data.clone());

        working_data = new_data;

        if write_out {
            let path_name: String = 
                SAMPLE_OUTPUT_RED.to_owned() + 
                &reductions.to_string() + 
                &String::from(".txt");

            working_data.write_to_file(path_name);
        }

        reductions += 1;
    };

    accumulate_reductions(&accumulative_data)
}

// --- //

const SAMPLE_INPUT_PATH: &str = "samplekanji.txt";
const SAMPLE_OUTPUT_CLEAR: &str = "sampleoutput.txt";
const SAMPLE_OUTPUT_RED: &str = "reduction#";
const SAMPLE_OUTPUT_ACC: &str = "accumulations.txt";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_get_sample_data() {
        let file_content = fs::read_to_string(SAMPLE_INPUT_PATH.to_string());
        for content in file_content {
            println!("{}", content);
        }
    }

    #[test]
    fn can_get_input_from_sample() {
        match textfile_to_int_vector(SAMPLE_INPUT_PATH.to_string()) {
            Ok(all_data) => assert_eq!(all_data.len(), 64 * 64),
            Err(error) => panic!("Input data couldn't be retrieved: {}", error),
        }
    }

    #[test]
    fn can_generate_sample_struct() {
        let sample_size = 64;

        let sample_data: Vmatrix<u32> = textfile_to_vmatrix(SAMPLE_INPUT_PATH.to_string(), sample_size);
        assert_eq!(sample_data.size, sample_size);
        assert_eq!(sample_data.data.len(), sample_size * sample_size);
    }

    #[test]
    fn bound_rows_zero() {
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

    #[test]
    fn can_write_back_into_file() {
        let sample_size = 64;

        let mut complete_message: String = String::from("");
        for i in 0..sample_size {
            for j in 0..sample_size {
                complete_message += &String::from("9");
            }    
            complete_message += &String::from("\n");
        }
        fs::write(SAMPLE_OUTPUT_CLEAR, complete_message);
    }

    #[test]
    fn write_out_sample_reductions() {
        let sample_size: usize = 64;

        let input_data: Vmatrix<u32> = textfile_to_vmatrix(SAMPLE_INPUT_PATH.to_string(), sample_size);

        let accumulations: Vmatrix<u32> = get_accumulation(input_data, false);
        accumulations.write_to_file(SAMPLE_OUTPUT_ACC.to_string());
    }
}
