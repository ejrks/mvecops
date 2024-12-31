use std::fmt;
use std::collections::HashSet;

use crate::Vector2;

use crate::get_coordinates_from;
use crate::sum_i64_vectors;
use crate::sub_vectors;
use crate::scale_vector;

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