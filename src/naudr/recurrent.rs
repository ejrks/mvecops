use crate::Vmatrix;

/// Preserve only entries that appear in a row [minimum_recursion]*times before skipping
/// to the next row, based on input_data size as a matrix.
///
pub fn recurrent_trace(input_data: &Vmatrix<u32>, minimum_recursion: usize) -> Vmatrix<u32> {
    let set_size: usize = input_data.data.len();

    let working_data = &input_data.data;
    let mut result: Vmatrix<u32> = Vmatrix::<u32>::initialize(input_data.size, 0);
    let working_result = &mut result.data;
 
    let mut anchor_index: usize = 0;
    let mut anchor_count: usize = 0;

    let mut recursion_found: bool = false;

    for i in 0..set_size {
        let zero_entry = working_data[i] == 0;

        if anchor_count == 0 && !zero_entry {
            anchor_index = i;
            anchor_count += 1;
            continue;
        }
        if zero_entry {
            anchor_count = 0;
            recursion_found = false;
        }
        if !recursion_found && !zero_entry {
            anchor_count += 1;
            if anchor_count >= minimum_recursion {
                recursion_found = true;
                let mut j: usize = anchor_index;
                while j != i {
                    working_result[j] = 1;
                    j += 1;
                }
            }
        }
        if recursion_found {
            working_result[i] = 1;
        }
    }

    result
}