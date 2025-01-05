use std::fs;

use crate::Vector2;

use crate::DefinitionUnit;
use crate::beorc::def::Trace;

#[derive(Clone)]
pub struct QuickTrace {
    pub id: String,

    pub trace: Vector2<i64>,
    pub average: Vector2<i64>,
}

impl QuickTrace {
    pub fn empty() -> QuickTrace {
        QuickTrace {
            id: String::from("Untreated trace"),
            trace: Vector2::new(0, 0),
            average: Vector2::new(0, 0),
        }
    }

    pub fn new(id: String, trace: Vector2<i64>, average: Vector2<i64>) -> QuickTrace {
        QuickTrace {
            id,
            trace,
            average,
        }
    }
}

pub struct TraceGroup {
    pub group_content: Vec<QuickTrace>
}

impl TraceGroup {
    pub fn empty() -> TraceGroup {
        TraceGroup {
            group_content: Vec::new(),
        }
    }
}

pub struct LivingDataUnit {
    pub definitions: Vec<DefinitionUnit>,

    pub trace_groups: Vec<TraceGroup>,
}

impl LivingDataUnit {
    pub fn empty() -> LivingDataUnit {
        LivingDataUnit {
            definitions: Vec::new(),
            trace_groups: Vec::new(),
        }
    }

    pub fn load_from_file(&mut self, quick_target: String, heavy_target: String, resolution: i64) -> bool {
        self.definitions = Vec::new();
        self.trace_groups = Vec::new();

        let heavy_target_content = fs::read_to_string(heavy_target);
        let quick_target_content = fs::read_to_string(quick_target);

        for content in heavy_target_content.expect("No HeavyDB found or maybe it was empty").lines() {
            let mut new_definition_unit = DefinitionUnit::new(resolution);

            let split_content: Vec<&str> = content.split(".").collect();
            new_definition_unit.id = (&split_content[0]).to_string();
            let mut time_stamp = 0;
            for split_traces in split_content[1].split(";") {
                let split_values: Vec<&str> = split_traces.split(",").collect();
                let mut trace_values: Vec<i64> = Vec::new();
                for entry in split_values {
                    match entry.parse::<i64>() {
                        Ok(value) => trace_values.push(value),
                        Err(_) => (),
                    }
                }
                if trace_values.len() > 0 {
                    new_definition_unit.feed(time_stamp, trace_values);
                }

                time_stamp += 1;
            }

            self.definitions.push(new_definition_unit);
        }

        let empty_line = String::from("");

        let mut current_index: usize = 0;
        let mut last_index: usize = 0;
        let mut even_entry: bool = true;

        for content in quick_target_content.expect("No QuickDB found or maybe it was empty").lines() {
            let mut new_trace_group = TraceGroup::empty();

            let collected_entries: Vec<&str> = content.split(".").collect();
            let mut selected_entry = collected_entries[0];

            current_index = 0;
            even_entry = true;
            last_index = collected_entries.len() - 1;
            let mut new_quick_trace: QuickTrace = QuickTrace::empty();

            while current_index < last_index {
                selected_entry = collected_entries[current_index];
                if even_entry {
                    new_quick_trace = QuickTrace::empty();
                    new_quick_trace.id = String::from(selected_entry);
                }
                else {
                    let split_traces = selected_entry.split(",");
                    let split_values: Vec<&str> = split_traces.collect();
                    let mut index_values: Vec<i64> = Vec::new();
                    for entry in split_values {
                        match entry.parse::<i64>() {
                            Ok(value) => index_values.push(value),
                            Err(_) => (),
                        }
                    }
                    new_quick_trace.trace = Vector2::new(index_values[0], index_values[1]);
                    new_quick_trace.average = Vector2::new(index_values[2], index_values[3]);
                    new_trace_group.group_content.push(new_quick_trace.clone());
                }

                current_index += 1;
                even_entry = current_index % 2 == 0;
            }

            self.trace_groups.push(new_trace_group);
        }

        for definition_found in &self.definitions {
            let trace_length = definition_found.traces.len();
            let mut current_index = 0;
            while current_index < trace_length {
                let heavy_trace = &definition_found.traces[current_index];
                for quick_trace in &self.trace_groups[current_index].group_content {
                    if quick_trace.id == definition_found.id {
                        if !quick_trace.trace.equals(&heavy_trace.trace) {
                            return false;
                        }
                        if !quick_trace.average.equals(&heavy_trace.average_offset) {
                            return false;
                        }
                    }
                }
                current_index += 1;
            }
        }

        return true;
    }

    pub fn dump_to_file(&self, append_name: String) {
        let quick_name = String::from("quickaccess_") + &append_name;
        let heavy_name = String::from("heavyaccess_") + &append_name;

        let mut quick_output = String::from("");
        let mut formatted_trace = String::from("");
        let comma_str = String::from(",");

        let mut current_check = 0;

        let mut skip_list: Vec<usize> = Vec::new();
        let definitions_size = self.definitions.len();
        let mut skip_size: usize = 0;

        let mut selected_definition: &DefinitionUnit = &DefinitionUnit::new(0);
        let mut selected_trace: &Trace = &Trace::empty();
        while skip_size < definitions_size {
            for x in 0..definitions_size {
                if !skip_list.contains(&x) {
                    selected_definition = &self.definitions[x];

                    if current_check >= selected_definition.traces.len() {
                        skip_list.push(x);
                        skip_size += 1;
                        continue;
                    }
                    
                    quick_output += &((selected_definition.id).clone() + &(String::from(".")));
                    
                    formatted_trace = String::from("");
                    selected_trace = &selected_definition.traces[current_check];
                    formatted_trace += &(selected_trace.trace.x.to_string() + &(comma_str)
                                    +   &selected_trace.trace.y.to_string() + &(comma_str)
                                    +   &selected_trace.average_offset.x.to_string() + &(comma_str)
                                    +   &selected_trace.average_offset.y.to_string() + &(comma_str)
                                    +   &(String::from(".")));
                    quick_output += &formatted_trace;
                }
            }
            current_check += 1;
            if skip_size < definitions_size {
                quick_output += &(String::from("\n"));
            }
        }

        fs::write(&quick_name, quick_output);

        let mut heavy_output = String::from("");
        let mut last_item: usize = 0;

        let mut trace_index: usize = 0;
        let mut trace_size: usize = 0;
 
        current_check = 0;
        while current_check < definitions_size {
            selected_definition = &self.definitions[current_check];
                    
            heavy_output += &((selected_definition.id).clone() + &(String::from(".")));
                    
            for trace in &selected_definition.traces {
                formatted_trace = String::from("");

                trace_size = trace.indexes.len();
                last_item = trace_size - 1;
                for x in 0..trace_size {
                    formatted_trace += &(trace.indexes[x].to_string());
                    if (x != last_item) {
                        formatted_trace += &(comma_str);
                    }
                }
                formatted_trace += &(String::from(";"));

                heavy_output += &formatted_trace;
            }

            heavy_output += &(String::from("\n"));
            current_check += 1;
        }

        fs::write(&heavy_name, heavy_output);
    }
}