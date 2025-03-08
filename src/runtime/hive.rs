// Implementation of the Hive multidimensional data structure

/// Represents a multidimensional Hive data structure
pub struct Hive {
    dimensions: Vec<usize>,
    data: Vec<f64>,
    is_circular: bool,
    is_persistent: bool,
}

impl Hive {
    /// Creates a new Hive with the specified dimensions
    pub fn new(dimensions: Vec<usize>, is_circular: bool, is_persistent: bool) -> Self {
        let size = dimensions.iter().product();
        let data = vec![0.0; size];
        
        Hive {
            dimensions,
            data,
            is_circular,
            is_persistent,
        }
    }
    
    /// Gets the total number of elements in the Hive
    pub fn size(&self) -> usize {
        self.data.len()
    }
    
    /// Gets the number of dimensions in the Hive
    pub fn dimension_count(&self) -> usize {
        self.dimensions.len()
    }
}