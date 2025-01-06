use crate::beorc::def::Trace;
use crate::LivingDataUnit;

use crate::cos_between;
use crate::close_enough_f64;

// To improve performance and usage, if the method is finalized, the dictionary check should be split
// into callable differentials, so it's faster

pub struct Medium {
    pub data_unit: LivingDataUnit,

    pub last_trace: Trace,

    pub predictions: Vec<Prediction>,

    pub current_best: String,
}

pub struct Prediction {
    pub id: String,
    pub likeness: f64,
}

impl Prediction {
    pub fn new(id: String, likeness: f64) -> Prediction {
        Prediction {
            id,
            likeness,
        }
    }
}

impl Medium {
    pub fn new(data_unit: LivingDataUnit) -> Medium {
        Medium {
            data_unit,
            last_trace: Trace::empty(),
            predictions: Vec::new(),
            current_best: String::from("bbeorrcc"),
        }
    }

    pub fn reset_search(&mut self) {
        self.last_trace = Trace::empty();
        self.predictions = Vec::new();
    }
    
    pub fn feed_trace(&mut self, new_trace: Trace) {
        self.last_trace = new_trace;
        self.update_search();
    }

    pub fn update_search(&mut self) {
        if self.last_trace.time_stamp == -1 {
            return;
        }

        let search_time_stamp = self.last_trace.time_stamp as usize;
        if search_time_stamp >= self.data_unit.trace_groups.len() {
            return;
        }

        let compare_trace = &self.last_trace.trace;
        let compare_average = &self.last_trace.average_offset;

        let mut partial_predictions: Vec<Prediction> = Vec::new();
        let mut best_value: f64 = -1.0;
        let mut worst_of_10: f64 = 1.0;
        let mut worst_of_count: usize = 0;

        let mut trace_likeness: f64 = 0.0;
        let mut average_likeness: f64 = 0.0;
        let mut total_likeness: f64 = 0.0;
        for entry in &self.data_unit.trace_groups[search_time_stamp].group_content {
            trace_likeness = cos_between(&compare_trace, &entry.trace);
            average_likeness = cos_between(&compare_average, &entry.average);

            if trace_likeness < 0.0 {
                trace_likeness = 0.0;
            }
            if average_likeness < 0.0 {
                average_likeness = 0.0;
            }
            total_likeness = trace_likeness + average_likeness;

            if total_likeness > best_value {
                best_value = total_likeness;
            }
            if worst_of_count < 10 && total_likeness < worst_of_10 {
                worst_of_10 = total_likeness;
                worst_of_count += 1;
            }

            partial_predictions.push(Prediction::new(entry.id.to_string(), total_likeness));
        }

        let mut filtered_predictions: Vec<Prediction> = Vec::new();
        for entry in partial_predictions {
            if entry.likeness > worst_of_10 {
                filtered_predictions.push(entry);
            }
        }
        self.update_predictions(filtered_predictions);
    }

    fn update_predictions(&mut self, new_predictions: Vec<Prediction>) {
        if self.predictions.len() == 0 {
            for entry in new_predictions {
                self.predictions.push(entry);
            }
            return;
        }
        else {
            let mut combined_predictions: Vec<Prediction> = Vec::new();
            
            let mut current_best_match: f64 = -1.0;
            
            let mut old_value = 0.0;
            let mut new_value = 0.0;
            let mut combined_value = 0.0;

            let mut current_trace: &String = &String::from("");

            let mut update_entry: bool = false;
            for entry in new_predictions {
                update_entry = false;
                current_trace = &entry.id;
                for old_entry in &self.predictions {
                    if *current_trace == old_entry.id {
                        old_value = old_entry.likeness;
                        new_value = entry.likeness;

                        combined_value = old_value + new_value;

                        if combined_value > current_best_match {
                            current_best_match = combined_value;
                            self.current_best = current_trace.to_string();
                        }

                        combined_predictions.push(Prediction::new(current_trace.to_string(), combined_value));
                        update_entry = true;
                    }
                }
                if !update_entry {
                    let time_stamp = self.last_trace.time_stamp;
                    combined_value = entry.likeness.powf(time_stamp as f64);
                    
                    if combined_value > current_best_match {
                        current_best_match = combined_value;
                        self.current_best = current_trace.to_string();
                    }

                    combined_predictions.push(Prediction::new(current_trace.to_string(), combined_value))
                }
            }

            self.predictions = combined_predictions;
        }
    }

    pub fn get_list_of_predictions(&self) -> (Vec<String>, Vec<f64>) {
        let mut result_id: Vec<String> = Vec::new();
        let mut result_values: Vec<f64> = Vec::new();

        let mut times: Vec<i64> = self.predictions.iter().map(|entry| (entry.likeness * 100.0) as i64).collect();
        times.sort();
        times.reverse();

        let mut times_index = 0;
        let predictions_size = self.predictions.len();
        let mut prediction_times: f64 = 0.0;
        while times_index < 10 && times_index < predictions_size {
            prediction_times = (times[times_index] as f64) / 100.0;
            for entry in &self.predictions {
                if close_enough_f64(entry.likeness, prediction_times, 0.01) {
                    result_id.push(entry.id.to_string());
                    result_values.push(entry.likeness);
                    times_index += 1;
                    continue;
                }
            }
            times_index += 1;
        }

        return (result_id, result_values);
    }
}

pub fn print_predictions(ids: Vec<String>, values: Vec<f64>) -> String {
    let mut result: String = String::from("");

    let mut current_index = 0;
    let maximum_index = ids.len();

    while current_index < maximum_index {
        result += &(ids[current_index]);
        result += &(String::from(" - "));
        result += &(values[current_index].to_string());
        result += &(String::from("\n"));
        current_index += 1;
    }

    return result;
}




