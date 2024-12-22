use std::fs;

trait Number {}
impl Number for u32 {}

/// Basic structure of mvecops, stores the size of a "row" and all the data in a generic vector
#[derive(Clone)]
pub struct Vmatrix<T>
where
    T: Clone,
{
    pub size: usize,
    pub data: Vec<T>,
}

impl<T> Vmatrix<T> 
where
    T: Clone + Copy,
{
    /// Create an empty instance of Vmatrix
    pub fn new(size: usize) -> Vmatrix<T> {
        Vmatrix {
            data: Vec::new(),
            size,
        }
    }

    /// Create a new Vmatrix with a starting value
    pub fn initialize(size: usize, initial_value: T) -> Vmatrix<T> {
        let mut new_instance = Vmatrix {
            data: Vec::new(),
            size,
        };

        let total_data_size = size * size;
        for x in 0..total_data_size {
             new_instance.data.push(initial_value.clone());
        }

        new_instance
    }

    /// Build a new Vmatrix using data from an existing standard vector
    pub fn build(size: usize, new_data: Vec<T>) -> Vmatrix<T> {
        let mut new_instance = Vmatrix {
            data: Vec::new(),
            size,
        };

        for entry in new_data {
             new_instance.data.push(entry);
        }

        new_instance
    }

    /// Rearrange the data so columns are read as files.
    pub fn transpose(&mut self) {
        let data_copy = self.data.clone();
        let size = self.size;

        for i in 0..size {
            for j in 0..size {
                self.data[i + j * self.size] = data_copy[j + i * size];
            }
        }
    }
}

impl<T> Vmatrix<T> 
where
    T: ToString + Clone,
{
    /// Write a whole Vmatrix to a file, inserting a new line after <size> number of characters were written
    pub fn write_to_file(&self, file_path: String) {
        let mut complete_output: String = String::from("");
        let size = self.size;

        for i in 0..size {
            for j in 0..size {
                complete_output += &self.data[j + i * size].to_string();
            }    
            complete_output += &String::from("\n");
        }
        fs::write(file_path, complete_output);
    }
}