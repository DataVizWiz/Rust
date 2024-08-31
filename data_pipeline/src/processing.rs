// Import necessary structures and enums
use crate::data::{DataEntry, DataValue};

// Function to filter entries based on a condition, returning references
pub fn filter_entries<'a>(entries: &'a Vec<DataEntry>, condition: &dyn Fn(&DataValue) -> bool) -> Vec<&'a DataEntry> {
    entries
        .iter()
        .filter(|entry| condition(&entry.value))
        .collect()
}

// Function to transform text values to uppercase
pub fn transform_text_to_uppercase(entries: &mut Vec<DataEntry>) {
    for entry in entries.iter_mut() {
        if let DataValue::Text(ref mut text) = entry.value {
            *text = text.to_uppercase();
        }
    }
}

// Function to summarize integer values by calculating the sum
pub fn sum_integer_values(entries: &Vec<DataEntry>) -> i32 {
    entries
        .iter()
        .map(|entry| {
            if let DataValue::Integer(val) = entry.value {
                val
            } else {
                0
            }
        })
        .sum()
}
