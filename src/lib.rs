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
use naudr::bloat::*;

// Remove usage of naudr without qualifying the name, common operations are being used and might
// overlap user api

// GENERIC //
pub fn textfile_to_int_vector(file_path: String) -> Result<Vec<u32>, Box<dyn Error>> {
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

pub fn textfile_to_vmatrix(file_path: String, size: usize) -> Vmatrix<u32> 
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

        let accumulations: Vmatrix<u32> = get_accumulation(&input_data, &output_path);
        accumulations.write_to_file(SAMPLE_OUTPUT_ACC.to_string());
}

/// Get the reductions from data at the target file. The input is a file with values either 1 or 0
/// that represent points where data is "found". The sample size is the number of elements per row
/// to recreate the data internally as a matrix. Use "samplekanji.txt" as reference.
/// 
pub fn get_accumulations_from_file(target_file: String, sample_size: usize) -> Vmatrix<u32> {
    let input_data: Vmatrix<u32> = textfile_to_vmatrix(target_file, sample_size);
    let output_path = None;
    get_accumulation(&input_data, &output_path)
}

/// See [get_accumulations_from_file]. This function is meant to be used with data directly, instead
/// of loading it through a file.
///
pub fn get_accumulations_from_data(input_data: Vmatrix<u32>, sample_size: usize) -> Vmatrix<u32> {
    let output_path = None;
    get_accumulation(&input_data, &output_path)
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

pub fn get_inflexions_from_vector(input_data: Vec<u32>, sample_size: usize, dominants_recurrency: usize) -> GlobalCurveData {
    let format_input_data = Vmatrix::build(sample_size, input_data);
    return get_complete_inflexions_from_data(&format_input_data, sample_size, dominants_recurrency);
}

pub fn get_complete_inflexions_from_data(input_data: &Vmatrix<u32>, sample_size: usize, dominants_recurrency: usize) -> GlobalCurveData {
    let mut global_curve_data = GlobalCurveData::new(sample_size);
    global_curve_data.transpose_internal();

    let output_path = None;
    let accumulations: Vmatrix<u32> = get_accumulation(&input_data, &output_path);

    let accumulations_transposed = accumulations.transposed_copy();

    let vertical_dominant = recurrent_trace(&accumulations, dominants_recurrency);
    let horizont_dominant = recurrent_trace(&accumulations_transposed, dominants_recurrency);

    let mut subtractions: Vmatrix<u32> = accumulations.normal_copy();
    subtractions = subtractions.xat(&vertical_dominant);
    subtractions.transpose();
    subtractions = subtractions.xat(&horizont_dominant);
    subtractions.transpose();

    let inflexion_curves = get_curves(&mut global_curve_data, &subtractions);

    let mut result_set_unclean = Vmatrix::initialize(sample_size, 0);
    mark_curve_points(&inflexion_curves, &mut result_set_unclean, &mut global_curve_data, false);

    let vertical_inflexion = get_curves(&mut global_curve_data, &vertical_dominant);

    let mut result_set_vertical = Vmatrix::initialize(sample_size, 0);
    mark_curve_points(&vertical_inflexion, &mut result_set_vertical, &mut global_curve_data, true);

    global_curve_data.transpose_internal();

    let horizontal_inflexion = get_curves(&mut global_curve_data, &horizont_dominant);

    let mut result_set_horizontal = Vmatrix::initialize(sample_size, 0);
    mark_curve_points(&horizontal_inflexion, &mut result_set_horizontal, &mut global_curve_data, true);

    return global_curve_data;
}

pub fn get_curve_no_reductions(input_data: Vec<u32>, sample_size: usize, dominants_recurrency: usize) -> GlobalCurveData {
    let format_input_data = Vmatrix::build(sample_size, input_data);
    return get_inflexions_no_reduction(&format_input_data, sample_size, dominants_recurrency);
}

pub fn get_inflexions_no_reduction(input_data: &Vmatrix<u32>, sample_size: usize, dominants_recurrency: usize) -> GlobalCurveData {
    let mut global_curve_data = GlobalCurveData::new(sample_size);
    global_curve_data.transpose_internal();

    let input_transposed = input_data.transposed_copy();

    let vertical_dominant = recurrent_trace(input_data, dominants_recurrency);
    let horizont_dominant = recurrent_trace(&input_transposed, dominants_recurrency);

    let mut subtractions: Vmatrix<u32> = input_data.normal_copy();
    subtractions = subtractions.xat(&vertical_dominant);
    subtractions.transpose();
    subtractions = subtractions.xat(&horizont_dominant);
    subtractions.transpose();

    let inflexion_curves = get_curves(&mut global_curve_data, &subtractions);

    let mut result_set_unclean = Vmatrix::initialize(sample_size, 0);
    mark_curve_points(&inflexion_curves, &mut result_set_unclean, &mut global_curve_data, false);

    let vertical_inflexion = get_curves(&mut global_curve_data, &vertical_dominant);

    let mut result_set_vertical = Vmatrix::initialize(sample_size, 0);
    mark_curve_points(&vertical_inflexion, &mut result_set_vertical, &mut global_curve_data, true);

    global_curve_data.transpose_internal();

    let horizontal_inflexion = get_curves(&mut global_curve_data, &horizont_dominant);

    let mut result_set_horizontal = Vmatrix::initialize(sample_size, 0);
    mark_curve_points(&horizontal_inflexion, &mut result_set_horizontal, &mut global_curve_data, true);

    return global_curve_data;
}

pub fn get_bloat_data(input_data: Vec<u32>, sample_size: usize) -> GlobalCurveData {
    let mut global_curve_data = GlobalCurveData::new(sample_size);

    let format_input_data = Vmatrix::build(sample_size, input_data);
    write_bloats(&mut global_curve_data, &format_input_data);
    
    return global_curve_data;
}

pub fn get_combined_data(input_data: Vec<u32>, sample_size: usize, dominants_recurrency: usize) -> (GlobalCurveData, GlobalCurveData) {
    let format_input_data = Vmatrix::build(sample_size, input_data);

    return get_dominant_plus_bloat(&format_input_data, sample_size, dominants_recurrency);
}

pub fn get_dominant_plus_bloat(input_data: &Vmatrix<u32>, sample_size: usize, dominants_recurrency: usize) -> (GlobalCurveData, GlobalCurveData) {
    let mut global_curve_data = GlobalCurveData::new(sample_size);
    global_curve_data.transpose_internal();

    // let output_path = None;
    // let accumulations: Vmatrix<u32> = get_accumulation(&input_data, &output_path);

    let input_transposed = input_data.transposed_copy();

    let vertical_dominant = recurrent_trace(&input_data, dominants_recurrency);
    let horizont_dominant = recurrent_trace(&input_transposed, dominants_recurrency);

    let mut subtractions: Vmatrix<u32> = input_data.normal_copy();
    subtractions = subtractions.xat(&vertical_dominant);
    subtractions.transpose();
    subtractions = subtractions.xat(&horizont_dominant);
    subtractions.transpose();

    // let inflexion_curves = get_curves(&mut global_curve_data, &subtractions);

    // let mut result_set_unclean = Vmatrix::initialize(sample_size, 0);
    // mark_curve_points(&inflexion_curves, &mut result_set_unclean, &mut global_curve_data, false);

    let vertical_inflexion = get_curves(&mut global_curve_data, &vertical_dominant);

    let mut result_set_vertical = Vmatrix::initialize(sample_size, 0);
    mark_curve_points(&vertical_inflexion, &mut result_set_vertical, &mut global_curve_data, true);

    global_curve_data.transpose_internal();

    let horizontal_inflexion = get_curves(&mut global_curve_data, &horizont_dominant);

    let mut result_set_horizontal = Vmatrix::initialize(sample_size, 0);
    mark_curve_points(&horizontal_inflexion, &mut result_set_horizontal, &mut global_curve_data, true);

    let mut bloat_curve_data = GlobalCurveData::new(sample_size);
    write_bloats(&mut bloat_curve_data, &subtractions);

    return (global_curve_data, bloat_curve_data);
}

// --- 0.2.1 --- //

// --- END OF API --- //

const SAMPLE_INPUT_PATH: &str = "samplekanji.txt";
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
}
