use std::fmt;
use std::collections::HashSet;

use std::fs;

use crate::Vector2;

use crate::get_coordinates_from;
use crate::sum_i64_vectors;
use crate::sub_vectors;
use crate::scale_vector;
use crate::cos_between;

const ERROR_FACTOR: f64 = 0.2;
const COS_ERROR: f64 = 0.86;
const COS_REST: f64 = 0.14;



#[derive(Clone)]
pub struct Trace {
    pub time_stamp: i64,

    pub indexes: Vec<i64>,

    pub trace: Vector2<i64>,
    pub average_offset: Vector2<i64>,
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
        };

        return new_trace;
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

pub struct CompatibilityReport {
    pub trace_within_range: bool,
    pub timing_rating: f64,
    pub vectors_similarity: f64,
    pub offsets_similarity: f64,

    pub diagnosis: bool,
}

impl CompatibilityReport {
    pub fn new() -> CompatibilityReport {
        CompatibilityReport {
            trace_within_range: false,
            timing_rating: 0.0,
            vectors_similarity: 0.0,
            offsets_similarity: 0.0,

            diagnosis: true,
        }
    }

    pub fn to_string(&self) -> String {
        let mut result_string = String::from("");
        let new_line = String::from("\n");

        result_string += &(String::from("Trace number in range: ") + &self.trace_within_range.to_string() + &new_line);
        result_string += &(String::from("Timing rating: ") + &self.timing_rating.to_string() + &new_line);
        result_string += &(String::from("Vector likeness: ") + &self.vectors_similarity.to_string() + &new_line);
        result_string += &(String::from("Offset likeness: ") + &self.offsets_similarity.to_string() + &new_line);

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

    pub fn feed(&mut self, new_instance: DefinitionUnit) {
        self.training_instances.push(new_instance);
    }

    pub fn train_w_report(&self) {
        if (self.training_instances.len() == 0) {
            panic!("There are no definition units for training. Cancelled.");
        }

        let mut report = String::from("");
        let new_line = String::from("\n");
        report += &(self.base.id.clone() + &new_line);
        report += &(String::from("*************") + &new_line);

        let data_size = self.base.resolution * self.base.resolution;
        let error_resolution = self.base.resolution as f64 * ERROR_FACTOR;
        let base_reinforcement = 1.00 / self.training_instances.len() as f64;
        report += &(String::from("Data size: ") + &data_size.to_string() + &new_line);
        report += &(String::from("Reinforcement value: ") + &base_reinforcement.to_string() + &new_line);
        report += &(String::from("Error resolution: ") + &error_resolution.to_string() + &new_line);
        report += &(String::from("*************") + &new_line);
        
        for entry in &self.training_instances {
            let compatibility_report = Self::report_compatibility(&self.base, entry, self.error_margin);
            
            report += &(String::from("Reporting: ") + &entry.id.to_string() + &new_line);
            report += &compatibility_report.to_string();
            report += &(String::from("*************") + &new_line);
        }

        fs::write(String::from("debug_report_data.txt"), report);
    }

    pub fn report_compatibility(base_unit: &DefinitionUnit, entry_unit: &DefinitionUnit, error_margin: f64) -> CompatibilityReport {
        let error_resolution = base_unit.resolution as f64 * ERROR_FACTOR;

        let mut reporting = CompatibilityReport::new();

        let maximum_index_entry = entry_unit.traces.len();
        let maximum_index_base = base_unit.traces.len();

        reporting.trace_within_range = maximum_index_entry  >= maximum_index_base &&
                                       maximum_index_entry  <= maximum_index_base * 2;
        
        // Here extra traces would need to be coupled
        // Concatenate ending of one with beggining of the other

        let mut entry_index_check = 0;
        let timing_base = 1.00 / maximum_index_entry as f64;
        let mut timing_value = 0.0;
        while (entry_index_check < maximum_index_entry && entry_index_check < maximum_index_base) {
            let ts_base = base_unit.traces[entry_index_check].time_stamp as f64;
            let ts_entr = entry_unit.traces[entry_index_check].time_stamp as f64;

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
            let trace_entr = entry_unit.traces[entry_index_check].trace;

            let cosine_value = cos_between(&trace_base, &trace_entr);
            if  cosine_value > COS_ERROR {                
                println!("Found cosine within range: {}", cosine_value);
                let inner_result_debug = timing_base * ((COS_REST - (1.0 - cosine_value)) / COS_REST);
                println!("Inner value {}", inner_result_debug);
                timing_value += timing_base * ((COS_REST - (1.0 - cosine_value)) / COS_REST);
            }

            entry_index_check += 1;
        }

        reporting.vectors_similarity = timing_value;

        entry_index_check = 0;
        timing_value = 0.0;
        while (entry_index_check < maximum_index_entry && entry_index_check < maximum_index_base) {
            let trace_base = base_unit.traces[entry_index_check].average_offset;
            let trace_entr = entry_unit.traces[entry_index_check].average_offset;

            let cosine_value = cos_between(&trace_base, &trace_entr);
            if  cosine_value > COS_ERROR {                
                println!("Found cosine within range: {}", cosine_value);
                let inner_result_debug = timing_base * ((COS_REST - (1.0 - cosine_value)) / COS_REST);
                println!("Inner value {}", inner_result_debug);
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

        return reporting;
    }
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
        for value in &all_values {
            while (last_painted_index < *value && last_painted_index < last_possible_index) {
                output += &String::from("**");
                
                last_painted_index += 1;
                if (last_painted_index % self.resolution == 0) {
                    output += &String::from("\n");
                }
            }
            if (last_painted_index == *value) {
                output += &value.to_string();
                if (*value < 10) {
                    output += &String::from(" ");
                }
            }
            last_painted_index += 1;
            if (last_painted_index % self.resolution == 0) {
                output += &String::from("\n");
            }
        }

        while (last_painted_index < last_possible_index) {
            output += &String::from("**");
                
            last_painted_index += 1;
            if (last_painted_index % self.resolution == 0) {
                output += &String::from("\n");
            }
        }

        write!(f, "{}", output)
    }
}