use std::fmt;
use std::collections::HashSet;

use std::fs;

use crate::Vector2;

use crate::get_coordinates_from;
use crate::sum_i64_vectors;
use crate::sub_vectors;
use crate::scale_vector;
use crate::cos_between;
use crate::close_enough_f64;

const ERROR_FACTOR: f64 = 0.2;
const COS_ERROR: f64 = 0.86;
const COS_REST: f64 = 0.14;

#[derive(Clone)]
pub struct Trace {
    pub time_stamp: i64,

    pub indexes: Vec<i64>,

    pub trace: Vector2<i64>,
    pub average_offset: Vector2<i64>,

    pub resolution: i64,
}

impl Trace {
    pub fn new(time_stamp: i64, indexes: Vec<i64>, resolution: i64) -> Trace {
        let first_index = indexes[0];
        let last_index = indexes[indexes.len() - 1];

        let first_displacement = get_coordinates_from(first_index, resolution);
        let last_displacement = get_coordinates_from(last_index, resolution);

        let trace_value = sub_vectors(&last_displacement, &first_displacement);
        
        let mut total_sum: Vector2<i64> = Vector2::new(0, 0);
        let mut element_count = 0;
        for entry in &indexes {
            let mut result = get_coordinates_from(*entry, resolution);
            total_sum = sum_i64_vectors(&total_sum, &result);

            element_count += 1;
        }

        total_sum = scale_vector(&total_sum, element_count);
        let aoffset_value = sub_vectors(&total_sum, &first_displacement);

        let mut new_trace = Trace {
            time_stamp,
            indexes,

            trace: trace_value,
            average_offset: aoffset_value,

            resolution,
        };

        return new_trace;
    }

    pub fn empty() -> Trace {
        Trace {
            time_stamp: -1,
            indexes: Vec::new(),
            trace: Vector2::new(0, 0),
            average_offset: Vector2::new(0, 0),
            resolution: 0,
        }
    }
}

#[derive(Clone)]
pub struct DefinitionUnit {
    pub id: String,
    pub resolution: i64,

    pub traces: Vec<Trace>,
}

impl DefinitionUnit {
    pub fn new(resolution: i64) -> DefinitionUnit {
        DefinitionUnit {
            id: String::from("bbeorrcc"),
            resolution,

            traces: Vec::new(),
        }
    }

    pub fn feed(&mut self, time_stamp: i64, indexes: Vec<i64>) {
        let new_trace = Trace::new(time_stamp, indexes, self.resolution);
        self.traces.push(new_trace);
    }
}

pub struct TrainingUnit {
    pub base: DefinitionUnit,

    pub training_instances: Vec<DefinitionUnit>,

    pub error_margin: f64,
}

pub struct ReconstructionReport {
    pub base: Vec<i64>,
    pub check: Vec<i64>,
    pub accepted: bool,
}

impl ReconstructionReport {
    pub fn new(base: Vec<i64>, check: Vec<i64>, accepted: bool) -> ReconstructionReport {
        ReconstructionReport {
            base,
            check,
            accepted,
        }
    }

    pub fn to_string(&self) -> String {
        let mut result_string = String::from("");
        let new_line = String::from("\n");

        result_string += &(String::from("??????????????????????") + &new_line);
        result_string += &(String::from("Tried to reconstruct: ") + &(format!("{:?}", &self.base)) + &new_line);
        result_string += &(String::from("Got: ") + &(format!("{:?}", &self.check)) + &new_line);
        result_string += &(String::from("ACCEPTED: ") + &self.accepted.to_string() + &new_line);
        result_string += &(String::from("??????????????????????") + &new_line);

        return result_string;
    }
}

pub struct CompatibilityReport {
    pub trace_within_range: bool,
    pub timing_rating: f64,
    pub vectors_similarity: f64,
    pub offsets_similarity: f64,

    pub diagnosis: bool,

    reconstruction_traces: Vec<ReconstructionReport>,

    reconstructed_instance: DefinitionUnit,
}

impl CompatibilityReport {
    pub fn new() -> CompatibilityReport {
        CompatibilityReport {
            trace_within_range: false,
            timing_rating: 0.0,
            vectors_similarity: 0.0,
            offsets_similarity: 0.0,

            diagnosis: true,

            reconstruction_traces: Vec::new(),

            reconstructed_instance: DefinitionUnit::new(0),
        }
    }

    pub fn to_string(&self) -> String {
        let mut result_string = String::from("");
        let new_line = String::from("\n");

        result_string += &(String::from("Trace number in range: ") + &self.trace_within_range.to_string() + &new_line);
        result_string += &(String::from("Timing rating: ") + &self.timing_rating.to_string() + &new_line);
        result_string += &(String::from("Vector likeness: ") + &self.vectors_similarity.to_string() + &new_line);
        result_string += &(String::from("Offset likeness: ") + &self.offsets_similarity.to_string() + &new_line);

        for element in &self.reconstruction_traces {
            result_string += &(*element.to_string());
        }

        result_string += &(String::from("Diagnosis: ") + &self.diagnosis.to_string() + &new_line);
        result_string += &(String::from("----------- ") + &new_line);

        return result_string;
    }
}

impl TrainingUnit {
    pub fn new(new_base: &DefinitionUnit, error_margin: f64) -> TrainingUnit {
        TrainingUnit {
            base: new_base.clone(),

            training_instances: Vec::new(),

            error_margin,
        }
    }

    pub fn empty(error_margin: f64) -> TrainingUnit {
        TrainingUnit {
            base: DefinitionUnit::new(0),

            training_instances: Vec::new(),

            error_margin,
        }
    }

    pub fn feed(&mut self, new_instance: DefinitionUnit) {
        self.training_instances.push(new_instance);
    }

    pub fn train_w_report(&mut self) -> DefinitionUnit {
        if (self.training_instances.len() == 0) {
            panic!("There are no definition units for training. Cancelled.");
        }

        let mut new_definition = DefinitionUnit::new(self.base.resolution);
        new_definition.id = self.base.id.clone();

        let mut report = String::from("");
        let new_line = String::from("\n");
        report += &(self.base.id.clone() + &new_line);
        report += &(String::from("*************") + &new_line);

        let data_size = (self.base.resolution * self.base.resolution) as usize;
        let error_resolution = self.base.resolution as f64 * ERROR_FACTOR;
        let mut base_reinforcement = 1.00 / self.training_instances.len() as f64;
        report += &(String::from("Data size: ") + &data_size.to_string() + &new_line);
        report += &(String::from("Reinforcement value: ") + &base_reinforcement.to_string() + &new_line);
        report += &(String::from("Error resolution: ") + &error_resolution.to_string() + &new_line);
        report += &(String::from("*************") + &new_line);
        
        let mut valid_instances: Vec<usize> = Vec::new();
        let mut instance_index: usize = 0;
        for entry in &mut self.training_instances {
            let compatibility_report = Self::report_compatibility(&self.base, entry, self.error_margin);
            
            report += &(String::from("Reporting: ") + &entry.id.to_string() + &new_line);
            report += &compatibility_report.to_string();

            *entry = compatibility_report.reconstructed_instance;

            report += &(String::from("*************") + &new_line);

            if compatibility_report.diagnosis {
                valid_instances.push(instance_index);
            }
            instance_index += 1;
        }

        base_reinforcement = 1.00 / valid_instances.len() as f64;
        let br2 = base_reinforcement * 2.0;

        let mut content_votes: Vec<f64> = vec![0.0; data_size];
        let mut vanguard_votes: Vec<f64> = vec![0.0; data_size];
        let mut rearguard_votes: Vec<f64> = vec![0.0; data_size];

        let mut trace_index = 0;
        let max_trace_index = self.base.traces.len();

        let mut base_trace: &Trace = &Trace::empty();
        let mut entry_trace: &Trace = &Trace::empty();
        while trace_index < max_trace_index {
            base_trace = &self.base.traces[trace_index];
            
            for entry in &mut content_votes {
                *entry = 0.0;
            }
            for entry in &mut vanguard_votes {
                *entry = 0.0;
            }
            for entry in &mut rearguard_votes {
                *entry = 0.0;
            }

            for index in &base_trace.indexes {
                content_votes[*index as usize] = 1.0;
            }
            vanguard_votes[base_trace.indexes[0] as usize] = 1.0;
            rearguard_votes[base_trace.indexes[base_trace.indexes.len() - 1] as usize] = 1.0;

            for element in &valid_instances {
                entry_trace = &self.training_instances[*element].traces[trace_index];

                for x in 0..data_size {
                    content_votes[x] -= base_reinforcement;
                    vanguard_votes[x] -= base_reinforcement;
                    rearguard_votes[x] -= base_reinforcement;
                }

                for index in &entry_trace.indexes {
                    content_votes[*index as usize] += br2;
                }
                vanguard_votes[entry_trace.indexes[0] as usize] += br2;
                rearguard_votes[entry_trace.indexes[entry_trace.indexes.len() - 1] as usize] += br2;
         
            }

            report += &(String::from("Training trace: ") + &self.base.id.to_string());
            report += &report_votes(&content_votes, &vanguard_votes, &rearguard_votes, self.base.resolution);
            report += &(String::from("*************") + &new_line);

            let trained_trace: Trace = Self::train_trace_with(base_trace, &content_votes, &vanguard_votes, &rearguard_votes);

            new_definition.traces.push(trained_trace);

            trace_index += 1;
        }

        report += &(String::from("Training complete... Old object is: ") + &new_line);
        report += &(format!("{}", self.base));
        report += &(String::from(".............") + &new_line);
        report += &(String::from("...New object is: ") + &new_line);
        report += &(format!("{}", new_definition));
        report += &(String::from(".............") + &new_line);
        report += &(String::from("Remember: Objects are not overwriten anywhere. Hang on the output. "));

        fs::write(String::from("debug_report_data.txt"), report);

        return new_definition;
    }

    fn train_trace_with(base_trace: &Trace, content: &Vec<f64>, vanguard: &Vec<f64>, rearguard: &Vec<f64>) -> Trace {
        let mut new_content: Vec<i64> = Vec::new();

        let mut index_check: usize = 0;
        let maximum_index = content.len();
        while index_check < maximum_index {
            if content[index_check] > 0.0 {
                new_content.push(index_check as i64);
            }
            index_check += 1;
        }

        index_check = 0;
        let mut current_value: f64 = -1.0;
        let mut greatest_value_found: f64 = -1.0;
        let mut index_greatest_value: usize = 0;
        let mut two_greatest_values: bool = false;
        let mut current_guard: i64 = base_trace.indexes[0];

        while index_check < maximum_index {
            current_value = vanguard[index_check];
            if close_enough_f64(current_value, greatest_value_found, 0.01) {
                two_greatest_values = true;
            }
            if current_value > greatest_value_found {
                greatest_value_found = current_value;
                index_greatest_value = index_check;
                two_greatest_values = false;
            }

            index_check += 1;
        }

        let mut new_vanguard = index_greatest_value as i64;
        if two_greatest_values {
            new_vanguard = current_guard;
        }

        index_check = 0;
        let mut current_value: f64 = -1.0;
        let mut greatest_value_found: f64 = -1.0;
        let mut index_greatest_value: usize = 0;
        let mut two_greatest_values: bool = false;
        let mut current_guard: i64 = base_trace.indexes[base_trace.indexes.len() - 1];

        while index_check < maximum_index {
            current_value = rearguard[index_check];
            if close_enough_f64(current_value, greatest_value_found, 0.01) {
                two_greatest_values = true;
            }
            if current_value > greatest_value_found {
                greatest_value_found = current_value;
                index_greatest_value = index_check;
                two_greatest_values = false;
            }

            index_check += 1;
        }

        let mut new_rearguard = index_greatest_value as i64;
        if two_greatest_values {
            new_rearguard = current_guard;
        }

        return Self::combine_into_trace(new_content, new_vanguard, new_rearguard, &base_trace.time_stamp, &base_trace.resolution);
    }

    fn combine_into_trace(content: Vec<i64>, vanguard: i64, rearguard: i64, time_stamp: &i64, resolution: &i64) -> Trace{
        let mut new_trace_data = vec![vanguard.clone()];
        for entry in content {
            if !new_trace_data.contains(&entry) && entry != rearguard {
                new_trace_data.push(entry);
            }
        }
        new_trace_data.push(rearguard);

        

        Trace::new(*time_stamp, new_trace_data, *resolution)
    }

    pub fn report_compatibility(base_unit: &DefinitionUnit, entry_unit: &DefinitionUnit, error_margin: f64) -> CompatibilityReport {
        let error_resolution = base_unit.resolution as f64 * ERROR_FACTOR;

        let mut reporting = CompatibilityReport::new();

        let maximum_index_entry = entry_unit.traces.len();
        let maximum_index_base = base_unit.traces.len();

        reporting.trace_within_range = maximum_index_entry  >= maximum_index_base &&
                                       maximum_index_entry  <= maximum_index_base * 2;
        
        // Here extra traces would need to be coupled
        let mut reconstructed_instance: DefinitionUnit = entry_unit.clone();
        if reporting.trace_within_range && maximum_index_entry != maximum_index_base {
            let reconstruction_result = reconstruct_traces(&base_unit, &entry_unit, &mut reporting);
            if reconstruction_result.traces.len() == base_unit.traces.len() {
                reconstructed_instance = reconstruction_result;
            }
        }

        let mut entry_index_check = 0;
        let timing_base = 1.00 / maximum_index_base as f64;
        let mut timing_value = 0.0;
        while (entry_index_check < maximum_index_entry && entry_index_check < maximum_index_base) {
            let ts_base = base_unit.traces[entry_index_check].time_stamp as f64;
            let ts_entr = reconstructed_instance.traces[entry_index_check].time_stamp as f64;

            let timing_difference = (ts_base - ts_entr).abs();
            if  timing_difference < error_resolution {                
                timing_value += timing_base * ((error_resolution - timing_difference) / error_resolution);
            }

            entry_index_check += 1;
        }

        reporting.timing_rating = timing_value;

        entry_index_check = 0;
        timing_value = 0.0;
        while (entry_index_check < maximum_index_entry && entry_index_check < maximum_index_base) {
            let trace_base = base_unit.traces[entry_index_check].trace;
            let trace_entr = reconstructed_instance.traces[entry_index_check].trace;

            let cosine_value = cos_between(&trace_base, &trace_entr);
            if  cosine_value > COS_ERROR {                
                let inner_result_debug = timing_base * ((COS_REST - (1.0 - cosine_value)) / COS_REST);
                timing_value += timing_base * ((COS_REST - (1.0 - cosine_value)) / COS_REST);
            }

            entry_index_check += 1;
        }

        reporting.vectors_similarity = timing_value;

        entry_index_check = 0;
        timing_value = 0.0;
        while (entry_index_check < maximum_index_entry && entry_index_check < maximum_index_base) {
            let trace_base = base_unit.traces[entry_index_check].average_offset;
            let trace_entr = reconstructed_instance.traces[entry_index_check].average_offset;

            let cosine_value = cos_between(&trace_base, &trace_entr);
            if  cosine_value > COS_ERROR {                
                let inner_result_debug = timing_base * ((COS_REST - (1.0 - cosine_value)) / COS_REST);
                timing_value += timing_base * ((COS_REST - (1.0 - cosine_value)) / COS_REST);
            }

            entry_index_check += 1;
        }

        reporting.offsets_similarity = timing_value;

        if reporting.timing_rating < error_margin {
            reporting.diagnosis = false;
        }
        if reporting.vectors_similarity < error_margin {
            reporting.diagnosis = false;
        }
        if reporting.offsets_similarity < error_margin {
            reporting.diagnosis = false;
        }
        if !reporting.trace_within_range {
            reporting.diagnosis = false;
        }

        reporting.reconstructed_instance = reconstructed_instance;

        return reporting;
    }
}

fn report_votes(content: &Vec<f64>, vanguard: &Vec<f64>, rearguard: &Vec<f64>, resolution: i64) -> String {
    let mut result_string = String::from("");
    let new_line = String::from("\n");

    result_string += &new_line;
    result_string += &(String::from("#########CONTENT##########"));
    result_string += &(single_vector_report(content, resolution));
    result_string += &(String::from("#########FIRST############"));
    result_string += &(single_vector_report(vanguard, resolution));
    result_string += &(String::from("#########LAST#############"));
    result_string += &(single_vector_report(rearguard, resolution));

    return result_string;
}

fn single_vector_report(content: &Vec<f64>, resolution: i64) -> String {
    let mut result_string = String::from("");
    let new_line = String::from("\n");

    let resolution_as_index = resolution as usize;

    result_string += &(new_line);
    for x in 0..resolution_as_index {
        for y in 0..resolution_as_index {
            result_string += &(format!("{:>5.2}", content[y + x * resolution_as_index]));
            result_string += &(String::from(" "));
        }
        result_string += &(new_line);
    }
    return result_string;
}

// It's quite possible that the inner loop doesn't even need to be a loop
fn reconstruct_traces(base_unit: &DefinitionUnit, entry_unit: &DefinitionUnit, reporting: &mut CompatibilityReport) -> DefinitionUnit {
    let mut result = DefinitionUnit::new(base_unit.resolution);

    let mut index_for_base = 0;
    let maximum_base_index = base_unit.traces.len();
    let mut index_for_entry = 0;
    let maximum_entry_index = entry_unit.traces.len();

    let mut canceled_internal = false;

    let mut first_entry: Trace = Trace::empty();
    let mut second_entry: Trace = Trace::empty();

    let mut combined_vector: Vec<i64> = Vec::new();
    let mut combined_entry: Trace = Trace::empty();

    let mut compare_against: &Trace = &Trace::empty();

    let mut traces_difference_1: f64 = 0.0;
    let mut offset_difference_1: f64 = 0.0;
    let mut elements_difference_1: i64 = 0;
    let mut traces_difference_2: f64 = 0.0;
    let mut offset_difference_2: f64 = 0.0;
    let mut elements_difference_2: i64 = 0;

    let mut second_fetched: bool = false;
    while index_for_entry < maximum_entry_index {
        if index_for_entry >= maximum_entry_index || index_for_base >= maximum_base_index {
            canceled_internal = true;
        }

        while !canceled_internal {
            second_fetched = false;

            compare_against = &base_unit.traces[index_for_base];

            first_entry = entry_unit.traces[index_for_entry].clone();
            if (index_for_entry + 1) < maximum_entry_index {
                second_entry = entry_unit.traces[index_for_entry + 1].clone();
                second_fetched = true;
            }
            // else {
                // if index_for_base < maximum_base_index {
                    // canceled_internal;
                    // break;
                // }
            // }

            if (second_fetched) {
                combined_vector = first_entry.indexes.iter().chain(second_entry.indexes.iter()).cloned().collect();

                combined_entry = Trace::new(0, combined_vector, compare_against.resolution);

                traces_difference_1 = cos_between(&compare_against.trace, &first_entry.trace);
                offset_difference_1 = cos_between(&compare_against.average_offset, &first_entry.average_offset);
                elements_difference_1 = (compare_against.indexes.len() as i64 - first_entry.indexes.len() as i64).abs();

                traces_difference_2 = cos_between(&compare_against.trace, &combined_entry.trace);
                offset_difference_2 = cos_between(&compare_against.average_offset, &combined_entry.average_offset);
                elements_difference_2 = (compare_against.indexes.len() as i64 - combined_entry.indexes.len() as i64).abs();
            
                // If all are valid, select the one with the best elements match
                if traces_difference_1 > COS_ERROR && offset_difference_1 > COS_ERROR &&
                   traces_difference_2 > COS_ERROR && offset_difference_2 > COS_ERROR {
                    if elements_difference_1 < elements_difference_2 {
                        first_entry.time_stamp = compare_against.time_stamp;
                        reporting.reconstruction_traces.push(ReconstructionReport::new(compare_against.indexes.clone(), first_entry.indexes.clone(), true));
                        result.traces.push(first_entry);

                        index_for_base += 1;
                        break;
                    }
                    else {
                        combined_entry.time_stamp = compare_against.time_stamp;
                        reporting.reconstruction_traces.push(ReconstructionReport::new(compare_against.indexes.clone(), combined_entry.indexes.clone(), true));
                        result.traces.push(combined_entry);

                        index_for_entry += 1;
                        index_for_base += 1;
                        break;
                    }
                }

                // If not, it may try to take the combined match right away if the number of elements are closer,
                // but only if its errors are within the margin
                if elements_difference_2 < elements_difference_1 {
                    if traces_difference_2 > COS_ERROR && offset_difference_2 > COS_ERROR {
                        combined_entry.time_stamp = compare_against.time_stamp;
                        reporting.reconstruction_traces.push(ReconstructionReport::new(compare_against.indexes.clone(), combined_entry.indexes.clone(), true));
                        result.traces.push(combined_entry);

                        index_for_entry += 1;
                        index_for_base += 1;
                        break;
                    }
                }

                // Finally, it may try to use the best match based on the errors
                let mut best_match_new = 0;
                if traces_difference_2 > traces_difference_1 {
                    best_match_new += 1;
                }
                else {
                    best_match_new -= 1;
                }

                if offset_difference_2 > offset_difference_1 {
                    best_match_new += 1;
                }
                else {
                    best_match_new -= 1;
                }

                if elements_difference_2 < elements_difference_1 {
                    best_match_new += 1;
                }
                else {
                    best_match_new -= 1;
                }

                if best_match_new > 0 {
                    let accepted: bool = traces_difference_2 > COS_ERROR && offset_difference_2 > COS_ERROR;
                    combined_entry.time_stamp = compare_against.time_stamp;
                    reporting.reconstruction_traces.push(ReconstructionReport::new(compare_against.indexes.clone(), combined_entry.indexes.clone(), accepted));
                    result.traces.push(combined_entry);

                    index_for_entry += 1;
                    index_for_base += 1;
                    break;
                }
                else {
                    let accepted: bool = traces_difference_1 > COS_ERROR && offset_difference_1 > COS_ERROR;
                    first_entry.time_stamp = compare_against.time_stamp;
                    reporting.reconstruction_traces.push(ReconstructionReport::new(compare_against.indexes.clone(), first_entry.indexes.clone(), accepted));
                    result.traces.push(first_entry);

                    index_for_base += 1;
                    break;
                }
            }
            else {  // No second entry fetched
                traces_difference_1 = cos_between(&compare_against.trace, &first_entry.trace);
                offset_difference_1 = cos_between(&compare_against.average_offset, &first_entry.average_offset);
                elements_difference_1 = (compare_against.indexes.len() as i64 - first_entry.indexes.len() as i64).abs();

                let mut match_check = 0;
                if traces_difference_1 > COS_ERROR {
                    match_check += 1;
                }
                if offset_difference_1 > COS_ERROR {
                    match_check += 1;
                }
                if (elements_difference_1 as f64) < (ERROR_FACTOR * compare_against.resolution as f64) {
                    match_check += 1;
                }

                first_entry.time_stamp = compare_against.time_stamp;
                reporting.reconstruction_traces.push(ReconstructionReport::new(compare_against.indexes.clone(), first_entry.indexes.clone(), match_check > 1));
                result.traces.push(first_entry);
                index_for_base += 1;
                break;
            }
        }        

        index_for_entry += 1;
    }

    return result;
}

impl fmt::Display for DefinitionUnit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut output = String::from("");
        let mut all_values_hs: HashSet<i64> = HashSet::new();
        for entry in &self.traces {
            for item in &entry.indexes {
                all_values_hs.insert(*item);
            }
        }

        let mut all_values = Vec::from_iter(all_values_hs);
        all_values.sort();

        let mut last_painted_index = 0;
        let last_possible_index = self.resolution * self.resolution;

        let longest_element = last_possible_index.to_string();
        let longest_element_number = longest_element.chars().count();
        let mut asterisk_string = String::from("");
        for x in 0..longest_element_number {
            asterisk_string += &(String::from("*"));
        }

        for value in &all_values {
            while (last_painted_index < *value && last_painted_index < last_possible_index) {
                output += &asterisk_string;
                
                last_painted_index += 1;
                if (last_painted_index % self.resolution == 0) {
                    output += &String::from("\n");
                }
            }
            if (last_painted_index == *value) {
                output += &(format!("{:1$}", value, longest_element_number));
            }
            last_painted_index += 1;
            if (last_painted_index % self.resolution == 0) {
                output += &String::from("\n");
            }
        }

        while (last_painted_index < last_possible_index) {
            output += &asterisk_string;
                
            last_painted_index += 1;
            if (last_painted_index % self.resolution == 0) {
                output += &String::from("\n");
            }
        }

        write!(f, "{}", output)
    }
}