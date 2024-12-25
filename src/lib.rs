use std::error::Error;

use std::vec::Vec;

use std::fs;

pub mod def;
pub mod naudr;
use def::vmatrix::*;
use def::trigonometric::*;
use def::maths::*;
use naudr::accumulate::*;
use naudr::recurrent::*;
use naudr::operate::*;
use naudr::closed_curves::*;

// Remove usage of naudr without qualifying the name, common operations are being used and might
// overlap user api

// GENERIC //
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
// API //

fn write_out_sample_reductions() {
        let sample_size: usize = 64;

        let input_data: Vmatrix<u32> = textfile_to_vmatrix(SAMPLE_INPUT_PATH.to_string(), sample_size);
  
        let output_path = Some(SAMPLE_OUTPUT_RED);

        let accumulations: Vmatrix<u32> = get_accumulation(input_data, &output_path);
        accumulations.write_to_file(SAMPLE_OUTPUT_ACC.to_string());
    }

/// Get the reductions from data at the target file. The input is a file with values either 1 or 0
/// that represent points where data is "found". The sample size is the number of elements per row
/// to recreate the data internally as a matrix. Use "samplekanji.txt" as reference.
/// 
pub fn get_accumulations_from_file(target_file: String, sample_size: usize) -> Vmatrix<u32> {
    let input_data: Vmatrix<u32> = textfile_to_vmatrix(target_file, sample_size);
    let output_path = None;
    get_accumulation(input_data, &output_path)
}

/// See [get_accumulations_from_file]. This function is meant to be used with data directly, instead
/// of loading it through a file.
///
pub fn get_accumulations_from_data(input_data: Vmatrix<u32>, sample_size: usize) -> Vmatrix<u32> {
    let output_path = None;
    get_accumulation(input_data, &output_path)
}

// Is sample_size ever used??
/// Get all the curves found in a set of data. A set of curves within a vector that is being treated
/// as a matrix is any body of data found in a curve closed from the rest of the data and that is not
/// made by completely vertical traces (a continuous column of filled data) or completely horizontal
/// data (a continuous row of filled data). This allows to find these straight lines, that can be
/// described trivially within a dataset, and separate them from any "non-elemental" line, that has a
/// heavier process to find a description.
///
pub fn get_substractions_from_data(accumulations: Vmatrix<u32>, sample_size: usize, dominants_recurrency: usize) -> Vmatrix<u32> {
    let accumulations_transposed = accumulations.transposed_copy();

    let vertical_dominant = recurrent_trace(&accumulations, dominants_recurrency);
    let horizont_dominant = recurrent_trace(&accumulations_transposed, dominants_recurrency);

    let mut substraction_result: Vmatrix<u32> = accumulations.normal_copy();
    substraction_result = substraction_result.xat(&vertical_dominant);
    substraction_result.transpose();
    substraction_result = substraction_result.xat(&horizont_dominant);
    substraction_result.transpose();

    return substraction_result;
}

// --- //

const SAMPLE_INPUT_PATH: &str = "samplekanji.txt";
const SAMPLE_OUTPUT_CLEAR: &str = "sampleoutput.txt";
const SAMPLE_OUTPUT_RED: &str = "reduction#";
const SAMPLE_OUTPUT_ACC: &str = "accumulations.txt";
const SAMPLE_OUTPUT_DOMVE: &str = "dominantVE.txt";
const SAMPLE_OUTPUT_DOMHR: &str = "dominantHR.txt";
const SAMPLE_OUTPUT_SUBST: &str = "subtraction.txt";
const SAMPLE_OUTPUT_INFLX: &str = "inflexion.txt";
const SAMPLE_OUTPUT_INFLX_VER: &str = "inflexion_ver.txt";
const SAMPLE_OUTPUT_INFLX_HOR: &str = "inflexion_hor.txt";

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
        let accumulations: Vmatrix<u32> = get_accumulations_from_file(SAMPLE_INPUT_PATH.to_string(), 64);
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
    fn transpose_clone() {
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

        let copied_data = data_sample.transposed_copy();
       
        assert_eq!(copied_data.data, sample_comp_5);
    }

    #[test]
    fn regular_clone() {
        let sample_data_5 = vec![1, 0, 1, 0, 1,
                                 1, 0, 1, 1, 0, 
                                 1, 1, 0, 0, 1, 
                                 1, 1, 0, 1, 1, 
                                 1, 0, 0, 0, 1];

        let mut data_sample = Vmatrix {
            size: 5,
            data: sample_data_5.clone(),
        };

        let copied_data = data_sample.normal_copy();
       
        assert_eq!(copied_data.data, sample_data_5);
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
    fn vmatrix_xat_different_size() {
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

        data_sample_1.xat(&data_sample_2);
    }

    #[test]
    fn vmatrix_xat() {
        let exclusive_at_data_5 = vec![1, 0, 1, 0, 1, 1, 0, 1, 1, 0, 1, 1, 0, 0, 1, 1, 1, 0, 1, 1, 1, 0, 0, 0, 1];
        let cannot_be_on_data_5 = vec![0, 0, 1, 0, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 1, 1, 0, 0, 1, 0, 1, 0, 1, 1, 1];
        let expected_result_dt5 = vec![1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0];

        let mut data_sample_1 = Vmatrix {
            size: 5,
            data: exclusive_at_data_5,
        };

        let mut data_sample_2 = Vmatrix {
            size: 5,
            data: cannot_be_on_data_5,
        };

        let result = data_sample_1.xat(&data_sample_2);
        assert_eq!(result.data, expected_result_dt5);
    }

    #[test]
    fn generate_substraction_curves() {
        let sample_size: usize = 64;
        let dominants_recurrency: usize = 12;

        let accumulations: Vmatrix<u32> = get_accumulations_from_file(SAMPLE_INPUT_PATH.to_string(), sample_size);
        let mut substraction_result = get_substractions_from_data(accumulations, sample_size, dominants_recurrency);

        substraction_result.write_to_file(SAMPLE_OUTPUT_SUBST.to_string());
    }

    #[test]
    fn trigonometrics_from_ints() {
        let cos_enum = Trigonometric::from_int(0);
        let neg_sin_enum = Trigonometric::from_int(1);
        let neg_cos_enum = Trigonometric::from_int(2);
        let sin_enum = Trigonometric::from_int(3);

        assert_eq!(cos_enum, Trigonometric::COS);
        assert_eq!(neg_sin_enum, Trigonometric::NSIN);
        assert_eq!(neg_cos_enum, Trigonometric::NCOS);
        assert_eq!(sin_enum, Trigonometric::SIN);
    }

    #[test]
    #[should_panic]
    fn on_unknown_trigonometric() {
        let failing_enum = Trigonometric::from_int(99);
    }

    #[test]
    fn trigonometrics_derivations() {
        let mut direction = Trigonometric::from_int(0);

        direction = Trigonometric::derivative(&direction);
        direction = Trigonometric::derivative(&Trigonometric::antiderivative(&direction));
        assert_eq!(direction, Trigonometric::NSIN);
        direction = Trigonometric::derivative(&direction);
        direction = Trigonometric::derivative(&Trigonometric::antiderivative(&direction));
        assert_eq!(direction, Trigonometric::NCOS);
        direction = Trigonometric::derivative(&Trigonometric::antiderivative(&direction));
        direction = Trigonometric::derivative(&direction);
        assert_eq!(direction, Trigonometric::SIN);
        direction = Trigonometric::derivative(&Trigonometric::antiderivative(&direction));
        direction = Trigonometric::derivative(&direction);
        assert_eq!(direction, Trigonometric::COS);

        direction = Trigonometric::antiderivative(&direction);
        assert_eq!(direction, Trigonometric::SIN);
        direction = Trigonometric::derivative(&Trigonometric::antiderivative(&direction));
        direction = Trigonometric::antiderivative(&direction);
        assert_eq!(direction, Trigonometric::NCOS);
        direction = Trigonometric::derivative(&Trigonometric::antiderivative(&direction));
        direction = Trigonometric::antiderivative(&direction);
        direction = Trigonometric::derivative(&Trigonometric::antiderivative(&direction));
        assert_eq!(direction, Trigonometric::NSIN);
        direction = Trigonometric::antiderivative(&direction);
        direction = Trigonometric::derivative(&Trigonometric::antiderivative(&direction));
        assert_eq!(direction, Trigonometric::COS);        
    }
  
    #[test]
    fn get_index_with_trigonometrics() {
        let starting_index: usize = 1502;
        let row_size: usize = 64;
        let mut result_index: usize = 0;

        result_index = Trigonometric::get_index_from_direction(starting_index, row_size, &Trigonometric::COS, 0);
        assert_eq!(result_index, 1503);
        result_index = Trigonometric::get_index_from_direction(starting_index, row_size, &Trigonometric::NCOS, 0);
        assert_eq!(result_index, 1501);

        result_index = Trigonometric::get_index_from_direction(starting_index, row_size, &Trigonometric::SIN, 0);
        assert_eq!(result_index, 1438);
        result_index = Trigonometric::get_index_from_direction(starting_index, row_size, &Trigonometric::NSIN, 0);
        assert_eq!(result_index, 1566);

        result_index = Trigonometric::get_index_from_direction(starting_index, row_size, &Trigonometric::COS, 1);
        assert_eq!(result_index, 1439);
        result_index = Trigonometric::get_index_from_direction(starting_index, row_size, &Trigonometric::NCOS, -1);
        assert_eq!(result_index, 1437);

        result_index = Trigonometric::get_index_from_direction(starting_index, row_size, &Trigonometric::SIN, -1);
        assert_eq!(result_index, 1439);
        result_index = Trigonometric::get_index_from_direction(starting_index, row_size, &Trigonometric::NSIN, 1);
        assert_eq!(result_index, 1567);
    }

    #[test]
    fn valid_index_on_vmatrixu32() {
        let exclusive_at_data_5 = vec![1, 0, 1, 0, 1, 1, 0, 1, 1, 0, 1, 1, 0, 0, 1, 1, 1, 0, 1, 1, 1, 0, 0, 0, 1];

        let mut data_sample_1 = Vmatrix {
            size: 5,
            data: exclusive_at_data_5,
        };

        assert_eq!(data_sample_1.test_index(12), true);
        assert_eq!(data_sample_1.test_index(1502), false);
    }

    #[test]
    fn get_first_inflexion_curve() {
        let sample_size: usize = 64;
        let dominants_recurrency = 12;

        let mut global_curve_data = GlobalCurveData::new(64);
        global_curve_data.transpose_internal();

        let accumulations: Vmatrix<u32> = get_accumulations_from_file(SAMPLE_INPUT_PATH.to_string(), sample_size);
        let mut subtractions = get_substractions_from_data(accumulations, sample_size, dominants_recurrency);

        let inflexion_curves = get_curves(&mut global_curve_data, &subtractions);
        // inflexion_curves.write_to_file(SAMPLE_OUTPUT_INFLX.to_string());

        let mut result_set_unclean = Vmatrix::initialize(sample_size, 0);
        mark_curve_points(&inflexion_curves, &mut result_set_unclean, &mut global_curve_data, false);
    }

    #[test]
    fn compute_index_distances() {
        let mut assertion = false;
        let mut result: f32 = 0.0;

        let error_margin = 0.01;
         
        result = get_index_distance(5,5,5);
        assert_eq!(close_enough(result, 0.0, error_margin), true);

        result = get_index_distance(1741, 2226, 64);
        assert_eq!(close_enough(result, 37.65, error_margin), true);  
        result = get_index_distance(2226, 1741, 64);
        assert_eq!(close_enough(result, 37.65, error_margin), true);  

        result = get_index_distance(0, 4095, 64);
        assert_eq!(close_enough(result, 89.09, error_margin), true);
        result = get_index_distance(4095, 0, 64);
        assert_eq!(close_enough(result, 89.09, error_margin), true);  

        result = get_index_distance(3990, 178, 64);
        assert_eq!(close_enough(result, 66.21, error_margin), true);  
        result = get_index_distance(178, 3990, 64);
        assert_eq!(close_enough(result, 66.21, error_margin), true); 

        result = get_index_distance(2519, 2524, 64);
        assert_eq!(close_enough(result, 5.0, error_margin), true);  
        result = get_index_distance(2524, 2519, 64);
        assert_eq!(close_enough(result, 5.0, error_margin), true); 

        result = get_index_distance(3962, 4026, 64);
        assert_eq!(close_enough(result, 1.0, error_margin), true);  
        result = get_index_distance(4026, 3962, 64);
        assert_eq!(close_enough(result, 1.0, error_margin), true); 
    }

    #[test]
    fn compute_middle_points() {
        let mut assertion = false;
        let mut result: usize = 0;

        let delta1: &mut Vector2<i32> = &mut Vector2::new(0, 0);
        let delta2: &mut Vector2<i32> = &mut Vector2::new(0, 0);

        result = get_middle_point(1741, 2226, 64, delta1, delta2);
        println!("VALUE DELTA1 [{}][{}]", delta1.x, delta1.y);
        println!("VALUE DELTA2 [{}][{}]", delta2.x, delta2.y);
        assert_eq!(result, 1951);
        result = get_middle_point(2226, 1741, 64, delta1, delta2);
        println!("VALUE DELTA1 [{}][{}]", delta1.x, delta1.y);
        println!("VALUE DELTA2 [{}][{}]", delta2.x, delta2.y);
        assert_eq!(result, 1951);

        result = get_middle_point(0, 4095, 64, delta1, delta2);
        println!("VALUE DELTA1 [{}][{}]", delta1.x, delta1.y);
        println!("VALUE DELTA2 [{}][{}]", delta2.x, delta2.y);
        assert_eq!(result, 2015);
        result = get_middle_point(4095, 0, 64, delta1, delta2);
        println!("VALUE DELTA1 [{}][{}]", delta1.x, delta1.y);
        println!("VALUE DELTA2 [{}][{}]", delta2.x, delta2.y);
        assert_eq!(result, 2015);

        result = get_middle_point(3990, 178, 64, delta1, delta2);
        println!("VALUE DELTA1 [{}][{}]", delta1.x, delta1.y);
        println!("VALUE DELTA2 [{}][{}]", delta2.x, delta2.y);
        assert_eq!(result, 2084);
        result = get_middle_point(178, 3990, 64, delta1, delta2);
        println!("VALUE DELTA1 [{}][{}]", delta1.x, delta1.y);
        println!("VALUE DELTA2 [{}][{}]", delta2.x, delta2.y);
        assert_eq!(result, 2084);
    }

    #[test]
    fn compute_orthogonal_vectors() {
        let mut input1: Vector2<i32> = Vector2::new(2, 3);
        let mut input2: Vector2<i32> = Vector2::new(-2, -3);
        let orthogonal1: &mut Vector2<i32> = &mut Vector2::new(0, 0);
        let orthogonal2: &mut Vector2<i32> = &mut Vector2::new(0, 0);

        orthogonal_from_antiparallel(&input1, &input2, orthogonal1, orthogonal2);
        assert_eq!((orthogonal1.x, orthogonal1.y), (-3, 2));
        assert_eq!((orthogonal2.x, orthogonal2.y), (3, -2));

        input1 = Vector2::new(0, 9);
        input2 = Vector2::new(0, -9);

        orthogonal_from_antiparallel(&input1, &input2, orthogonal1, orthogonal2);
        assert_eq!((orthogonal1.x, orthogonal1.y), (-9, 0));
        assert_eq!((orthogonal2.x, orthogonal2.y), (9, 0));

        input1 = Vector2::new(1, -10);
        input2 = Vector2::new(-1, 10);

        orthogonal_from_antiparallel(&input1, &input2, orthogonal1, orthogonal2);
        assert_eq!((orthogonal1.x, orthogonal1.y), (10, 1));
        assert_eq!((orthogonal2.x, orthogonal2.y), (-10, -1));
    }

    #[test]
    #[should_panic]
    fn orthogonal_from_not_antiparallel() {
        let input1: Vector2<i32> = Vector2::new(1, 3);
        let input2: Vector2<i32> = Vector2::new(-2, -3);
        let orthogonal1: &mut Vector2<i32> = &mut Vector2::new(0, 0);
        let orthogonal2: &mut Vector2<i32> = &mut Vector2::new(0, 0);
    
        orthogonal_from_antiparallel(&input1, &input2, orthogonal1, orthogonal2);
    }

    #[test]
    fn close_enough_checks() {
        assert_eq!(close_enough(0.0, 0.0, 0.0), true);
        assert_eq!(close_enough(0.1, 0.0, 0.0), false);
        assert_eq!(close_enough(0.1, 0.0, 0.1), false);
        assert_eq!(close_enough(-0.1, 0.0, 0.15), true);
        assert_eq!(close_enough(-0.1, 0.1, -0.21), true);
    }

    #[test]
    fn index_to_coordinates() {
        let row_size = 64;
        
        let mut result = get_index_as_coordinates(0, row_size);
        assert_eq!((result.x, result.y), (0, 0));
        let mut result = get_index_as_coordinates(3243, row_size);
        assert_eq!((result.x, result.y), (43, 50));
        let mut result = get_index_as_coordinates(501, row_size);
        assert_eq!((result.x, result.y), (53, 7));
        let mut result = get_index_as_coordinates(4095, row_size);
        assert_eq!((result.x, result.y), (63, 63));
    }

    #[test]
    fn sum_of_vectors() {
        let mut input1: Vector2<i32> = Vector2::new(1, 3);
        let mut input2: Vector2<i32> = Vector2::new(-2, -3);
        let mut result = sum_vectors(&input1, &input2);
        assert_eq!((result.x, result.y), (-1, 0));

        let mut input1: Vector2<i32> = Vector2::new(0, 0);
        let mut input2: Vector2<i32> = Vector2::new(-2, -3);
        let mut result = sum_vectors(&input1, &input2);
        assert_eq!((result.x, result.y), (-2, -3));

        let mut input1: Vector2<i32> = Vector2::new(1, 3);
        let mut input2: Vector2<i32> = Vector2::new(99, 77);
        let mut result = sum_vectors(&input1, &input2);
        assert_eq!((result.x, result.y), (100, 80));

        let mut input1: Vector2<i32> = Vector2::new(-1, -1);
        let mut input2: Vector2<i32> = Vector2::new(-8, -8);
        let mut result = sum_vectors(&input1, &input2);
        assert_eq!((result.x, result.y), (-9, -9));
    }

    #[test]
    fn displace_index_with_vectors() {
        let row_size = 64; 
 
        let mut input: usize = 0;
        let mut direction: Vector2<i32> = Vector2::new(10, 0);
        let mut result = array_position_vector_displacement(input, row_size, &direction);
        assert_eq!(result, 10);

        input = 396;
        direction = Vector2::new(5, 12);
        result = array_position_vector_displacement(input, row_size, &direction);
        assert_eq!(result, 1169);

        input = 1169;
        direction = Vector2::new(-5, -12);
        result = array_position_vector_displacement(input, row_size, &direction);
        assert_eq!(result, 396);

        input = 1169;
        direction = Vector2::new(-9, 4);
        result = array_position_vector_displacement(input, row_size, &direction);
        assert_eq!(result, 1416);

        input = 1416;
        direction = Vector2::new(20, -15);
        result = array_position_vector_displacement(input, row_size, &direction);
        assert_eq!(result, 476);

        input = 0;
        direction = Vector2::new(63, 0);
        result = array_position_vector_displacement(input, row_size, &direction);
        input = result as usize;
        direction = Vector2::new(0, 63);
        result = array_position_vector_displacement(input, row_size, &direction);
        input = result as usize;
        assert_eq!(input, 4095);
        direction = Vector2::new(-63, 0);
        result = array_position_vector_displacement(input, row_size, &direction);
        input = result as usize;
        direction = Vector2::new(0, -63);
        result = array_position_vector_displacement(input, row_size, &direction);
        assert_eq!(result, 0);
    }

    #[test]
    fn get_complete_inflexions() {
        let sample_size: usize = 64;
        let dominants_recurrency = 12;

        let mut global_curve_data = GlobalCurveData::new(64);
        global_curve_data.transpose_internal();

        let accumulations: Vmatrix<u32> = get_accumulations_from_file(SAMPLE_INPUT_PATH.to_string(), sample_size);

        let accumulations_transposed = accumulations.transposed_copy();

        let vertical_dominant = recurrent_trace(&accumulations, dominants_recurrency);
        let horizont_dominant = recurrent_trace(&accumulations_transposed, dominants_recurrency);

        let mut subtractions: Vmatrix<u32> = accumulations.normal_copy();
        subtractions = subtractions.xat(&vertical_dominant);
        subtractions.transpose();
        subtractions = subtractions.xat(&horizont_dominant);
        subtractions.transpose();

        let inflexion_curves = get_curves(&mut global_curve_data, &subtractions);
        inflexion_curves.write_to_file(SAMPLE_OUTPUT_INFLX.to_string());

        let mut result_set_unclean = Vmatrix::initialize(sample_size, 0);
        mark_curve_points(&inflexion_curves, &mut result_set_unclean, &mut global_curve_data, false);

        let vertical_inflexion = get_curves(&mut global_curve_data, &vertical_dominant);
        vertical_inflexion.write_to_file(SAMPLE_OUTPUT_INFLX_VER.to_string());

        let mut result_set_vertical = Vmatrix::initialize(sample_size, 0);
        mark_curve_points(&vertical_inflexion, &mut result_set_vertical, &mut global_curve_data, true);

        global_curve_data.transpose_internal();
        let horizontal_inflexion = get_curves(&mut global_curve_data, &horizont_dominant);
        horizontal_inflexion.write_to_file(SAMPLE_OUTPUT_INFLX_HOR.to_string());

        let mut result_set_horizontal = Vmatrix::initialize(sample_size, 0);
        mark_curve_points(&horizontal_inflexion, &mut result_set_horizontal, &mut global_curve_data, true);

        // global_curve_data.curves_global_output.write_to_file(String::from("global_output.txt"));
        // global_curve_data.curves_global_orderd.write_to_file(String::from("global_orderd.txt"));
    }

}
