use std::error::Error;

use std::vec::Vec;

use std::fs;

pub mod def;
pub mod naudr;
pub mod beorc;

use def::vmatrix::*;
use def::trigonometric::*;
use def::maths::*;
use naudr::accumulate::*;
use naudr::recurrent::*;
use naudr::operate::*;
use naudr::closed_curves::*;
use naudr::bloat::*;

use beorc::def::DefinitionUnit;
use beorc::def::TrainingUnit;

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
    fn trace_values_on_feed() {
        let mut dunit_sample: DefinitionUnit = DefinitionUnit::new(5);
        let trace_1: Vec<i64> = vec![6, 7, 8];
        let trace_2: Vec<i64> = vec![14, 18, 23];
        let trace_3: Vec<i64> = vec![5, 10, 15, 20];
        
        dunit_sample.feed(0, trace_1);
        dunit_sample.feed(1, trace_2);
        dunit_sample.feed(2, trace_3);

        assert_eq!(3, dunit_sample.traces[0].indexes.len());
        assert_eq!((2, 0), (dunit_sample.traces[0].trace.x, dunit_sample.traces[0].trace.y));
        assert_eq!((1, 0), (dunit_sample.traces[0].average_offset.x, dunit_sample.traces[0].average_offset.y));

        assert_eq!(3, dunit_sample.traces[1].indexes.len());
        assert_eq!((-1, 2), (dunit_sample.traces[1].trace.x, dunit_sample.traces[1].trace.y));
        assert_eq!((-1, 1), (dunit_sample.traces[1].average_offset.x, dunit_sample.traces[1].average_offset.y));

        assert_eq!(4, dunit_sample.traces[2].indexes.len());
        assert_eq!((0, 3), (dunit_sample.traces[2].trace.x, dunit_sample.traces[2].trace.y));
        assert_eq!((0, 1), (dunit_sample.traces[2].average_offset.x, dunit_sample.traces[2].average_offset.y));
    }

    #[test]
    fn print_definition_unit() {
        let mut dunit_sample: DefinitionUnit = DefinitionUnit::new(5);
        let trace_1: Vec<i64> = vec![6, 7, 8];
        let trace_2: Vec<i64> = vec![14, 18, 23];
        let trace_3: Vec<i64> = vec![5, 10, 15, 20];

        dunit_sample.feed(0, trace_1);
        dunit_sample.feed(1, trace_2);
        dunit_sample.feed(2, trace_3);
        
        println!("{}", dunit_sample);
    }

    #[test]
    #[should_panic]
    fn empty_tunit_panic() {
        let mut dunit_sample: DefinitionUnit = DefinitionUnit::new(5);
        dunit_sample.feed(0, vec![6, 7, 8]);
        dunit_sample.feed(1, vec![14, 18, 23]);
        dunit_sample.feed(2, vec![5, 10, 15, 20]);

        let mut tunit_sample: TrainingUnit = TrainingUnit::new(&dunit_sample, 0.5);
        tunit_sample.train_w_report();
    }

    #[test]
    fn single_training_session() {
        let mut dunit_sample: DefinitionUnit = DefinitionUnit::new(5);
        dunit_sample.feed(0, vec![6, 7, 8]);
        dunit_sample.feed(1, vec![14, 18, 23]);
        dunit_sample.feed(2, vec![5, 10, 15, 20]);

        let mut tunit_sample: TrainingUnit = TrainingUnit::new(&dunit_sample, 0.5);

        //  *** Case covers ***
        //  + Very similar, but one trace tries to correct a single index
        //  + Not even enough traces, discarded
        //  + Too many traces, but can adapt based on time stamps, tries to correct another index
        // (MISSING THIS ONE, IT IS IMPORTANT) Too many traces, attempts to introduce new one
        //  + Enough traces, but non of the offsets make sense, discarded
        //  + Enough traces, but one of them seems completely different, discarded
        //  + Whole system was raised one row, should recognize, tries to correct an index but keeps an old one
        //  + Exact same data, feeds it to be static
        //  *** Case covers ***

        let mut t1_same_data: DefinitionUnit = DefinitionUnit::new(5);
        t1_same_data.id = String::from("Same data");
        t1_same_data.feed(0, vec![6, 7, 8]);
        t1_same_data.feed(1, vec![14, 18, 23]);
        t1_same_data.feed(2, vec![5, 10, 15, 20]);
        tunit_sample.training_instances.push(t1_same_data);

        let mut t2_bad_data: DefinitionUnit = DefinitionUnit::new(5);
        t2_bad_data.id = String::from("Bad data");
        t2_bad_data.feed(0, vec![11, 16, 21, 22]);
        t2_bad_data.feed(1, vec![18, 24]);
        t2_bad_data.feed(2, vec![14, 9, 4]);
        tunit_sample.training_instances.push(t2_bad_data);

        let mut t3_same_data_time1: DefinitionUnit = DefinitionUnit::new(5);
        t3_same_data_time1.id = String::from("Same data time + 1");
        t3_same_data_time1.feed(0, vec![6, 7, 8]);
        t3_same_data_time1.feed(1, vec![14, 18, 23]);
        t3_same_data_time1.feed(3, vec![5, 10, 15, 20]);
        tunit_sample.training_instances.push(t3_same_data_time1);

        let mut t3_same_bad_timing: DefinitionUnit = DefinitionUnit::new(5);
        t3_same_bad_timing.id = String::from("Same data bad timing");
        t3_same_bad_timing.feed(6, vec![6, 7, 8]);
        t3_same_bad_timing.feed(3, vec![14, 18, 23]);
        t3_same_bad_timing.feed(7, vec![5, 10, 15, 20]);
        tunit_sample.training_instances.push(t3_same_bad_timing);

        let mut t4_correction1: DefinitionUnit = DefinitionUnit::new(5);
        t4_correction1.id = String::from("Correction data 1 off");
        t4_correction1.feed(0, vec![6, 7, 8, 3]);
        t4_correction1.feed(1, vec![14, 18, 23]);
        t4_correction1.feed(2, vec![5, 10, 15, 20]);
        tunit_sample.training_instances.push(t4_correction1);

        let mut t5_broken_offset: DefinitionUnit = DefinitionUnit::new(5);
        t5_broken_offset.id = String::from("Good trace bad ave");
        t5_broken_offset.feed(0, vec![6, 11, 16, 12, 18]);
        t5_broken_offset.feed(1, vec![14, 18, 19, 24, 23]);
        t5_broken_offset.feed(2, vec![0, 5, 10, 15, 20]);
        tunit_sample.training_instances.push(t5_broken_offset);

        let mut t6_allup: DefinitionUnit = DefinitionUnit::new(5);
        t6_allup.id = String::from("All up");
        t6_allup.feed(0, vec![1, 2, 3]);
        t6_allup.feed(1, vec![9, 13, 18]);
        t6_allup.feed(2, vec![0, 5, 10, 15]);
        tunit_sample.training_instances.push(t6_allup);

        let mut t7_split: DefinitionUnit = DefinitionUnit::new(5);
        t7_split.id = String::from("Split times");
        t7_split.feed(0, vec![6, 7]);
        t7_split.feed(1, vec![8]);
        t7_split.feed(2, vec![14]);
        t7_split.feed(3, vec![18, 23]);
        t7_split.feed(4, vec![5, 10]);
        t7_split.feed(5, vec![15, 20]);
        tunit_sample.training_instances.push(t7_split);

        let mut t8_hsplit: DefinitionUnit = DefinitionUnit::new(5);
        t8_hsplit.id = String::from("Split times - 1");
        t8_hsplit.feed(0, vec![6, 7, 8]);
        t8_hsplit.feed(1, vec![14]);
        t8_hsplit.feed(2, vec![18, 23]);
        t8_hsplit.feed(3, vec![5, 10, 15, 20]);
        tunit_sample.training_instances.push(t8_hsplit);

        tunit_sample.train_w_report();
    }

    #[test]
    fn cosine_calculation() {
        let mut vector1: Vector2<i64> = Vector2::new(3, 0);
        let mut vector2: Vector2<i64> = Vector2::new(4, 1);
        let mut result = cos_between(&vector1, &vector2);
        assert_eq!(close_enough(result as f32, 0.97, 0.02), true);

        vector1 = Vector2::new(3, 0);
        vector2 = Vector2::new(0, 3);
        result = cos_between(&vector1, &vector2);
        assert_eq!(close_enough(result as f32, 0.0, 0.02), true);

        vector1 = Vector2::new(3, 0);
        vector2 = Vector2::new(-2, 0);
        result = cos_between(&vector1, &vector2);
        assert_eq!(close_enough(result as f32, -1.0, 0.02), true);

        vector1 = Vector2::new(62, -93);
        vector2 = Vector2::new(92, -61);
        result = cos_between(&vector1, &vector2);
        assert_eq!(close_enough(result as f32, 0.92, 0.02), true);
    }
}
