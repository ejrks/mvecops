use std::error::Error;

use std::vec::Vec;

use std::fs;

pub mod def;
pub mod naudr;
use def::vmatrix::*;
use naudr::accumulate::*;
use naudr::recurrent::*;
use naudr::operate::*;

// Send to general, too generic
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

fn textfile_to_vmatrix(file_path: String, size: usize) -> Vmatrix<u32> 
{
    match textfile_to_int_vector(file_path) {
        Err(error) => panic!("Input data couldn't be retrieved: {}", error),
        Ok(all_data) => {
            Vmatrix::<u32>::build(size, all_data)
        }
    }
}

// --- //

const SAMPLE_INPUT_PATH: &str = "samplekanji.txt";
const SAMPLE_OUTPUT_CLEAR: &str = "sampleoutput.txt";
const SAMPLE_OUTPUT_RED: &str = "reduction#";
const SAMPLE_OUTPUT_ACC: &str = "accumulations.txt";
const SAMPLE_OUTPUT_DOMVE: &str = "dominantVE.txt";
const SAMPLE_OUTPUT_DOMHR: &str = "dominantHR.txt";

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
  
        let output_path = Some(SAMPLE_OUTPUT_RED);

        let accumulations: Vmatrix<u32> = get_accumulation(input_data, &output_path);
        accumulations.write_to_file(SAMPLE_OUTPUT_ACC.to_string());
    }

    #[test]
    fn transpose_data() {
        let sample_data_5 = vec![1, 0, 1, 0, 1,
                                 1, 0, 1, 1, 0, 
                                 1, 1, 0, 0, 1, 
                                 1, 1, 0, 1, 1, 
                                 1, 0, 0, 0, 1];

        let sample_comp_5 = vec![1, 1, 1, 1, 1,
                                 0, 0, 1, 1, 0, 
                                 1, 1, 0, 0, 0, 
                                 0, 1, 0, 1, 0, 
                                 1, 0, 1, 1, 1];

        let mut data_sample = Vmatrix {
            size: 5,
            data: sample_data_5,
        };

        data_sample.transpose();

        assert_eq!(data_sample.data, sample_comp_5);
    }

    #[test]
    fn dominant_directions() {
        let sample_size: usize = 64;

        let mut input_data: Vmatrix<u32> = textfile_to_vmatrix(SAMPLE_INPUT_PATH.to_string(), sample_size);

        let dominant_vertical: Vmatrix<u32> = recurrent_trace(&input_data, 12);
        dominant_vertical.write_to_file(SAMPLE_OUTPUT_DOMVE.to_string());

        input_data.transpose();

        let dominant_horizontal: Vmatrix<u32> = recurrent_trace(&input_data, 12);
        dominant_horizontal.write_to_file(SAMPLE_OUTPUT_DOMHR.to_string());         
    }

    #[test]
    #[should_panic]
    fn vmatrix_xor_different_size() {
        let sample_data_4 = vec![1, 0, 1, 0, 1, 1, 0, 1, 1, 0, 1, 1, 0, 0, 1, 1];
        let sample_data_5 = vec![1, 0, 1, 0, 1, 1, 0, 1, 1, 0, 1, 1, 0, 0, 1, 1, 1, 0, 1, 1, 1, 0, 0, 0, 1];

        let mut data_sample_1 = Vmatrix {
            size: 4,
            data: sample_data_4,
        };

        let mut data_sample_2 = Vmatrix {
            size: 5,
            data: sample_data_5,
        };

        data_sample_1.xor(data_sample_2);
    }

    #[test]
    fn vmatrix_xor() {
        
    }
}
