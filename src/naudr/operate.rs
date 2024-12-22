use crate::Vmatrix;

impl<T> Vmatrix<T>
where
    T: Clone + std::cmp::PartialEq<u32>,
{
    // Could be refactored to take closures and thus operate over any criterion, instead of just numbers
    /// Return a new Vmatrix that has 1 on the same entries where self had a number different from zero
    /// but other did NOT have an entry different from zero
    /// 
    /// #Panics
    ///
    /// The function panics if the inputs have different data length
    ///
    pub fn xor(&self, other: Vmatrix<u32>) -> Vmatrix<u32> {
        let set_length_1: usize = self.data.len();
        let set_length_2: usize = other.data.len();

        if set_length_1 != set_length_2 {
            panic!("This operation is impossible on data sets of different length. 
                    The found lengths are {set_length_1} and {set_length_2}");
        }

        let mut result: Vmatrix<u32> = Vmatrix::<u32>::initialize(self.size, 0);
        let working_result = &mut result.data;
        let data_self = &self.data;
        let data_othr = &other.data;

        for i in 0..set_length_1 {
            if (data_self[i] != 0) && (data_othr[i] == 0) {
                working_result[i] = 1;
            } else {
                working_result[i] = 0;
            }
        }

        result
    }
}