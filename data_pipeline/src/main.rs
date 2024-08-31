use data_pipeline::data::{DataEntry, DataValue};
use data_pipeline::processing::{filter_entries, sum_integer_values};

fn main() {
    let entries = vec![
        DataEntry::new(1, DataValue::Integer(42), "sensor1", "2024-08-31T12:00:00", vec!["temperature".to_string(), "indoors".to_string()]),
        DataEntry::new(2, DataValue::Text("hello world".to_string()), "sensor2", "2024-08-31T12:10:00", vec!["greeting".to_string()]),
    ];

    // Attempt to filter entries (immutable borrow)
    let filtered_entries = filter_entries(&entries, &|value| match value {
        DataValue::Integer(x) => *x > 10,
        _ => false,
    });

    // Attempt to transform text (mutable borrow)
    // transform_text_to_uppercase(&mut entries);

    // Attempt to calculate sum of integers (immutable borrow)
    let sum = sum_integer_values(&entries);

    // Display results
    println!("Filtered Entries:");
    for entry in &filtered_entries {
        entry.display();
    }

    println!("\nTransformed Entries:");
    for entry in &entries {
        entry.display();
    }

    println!("\nSum of integer values: {}", sum);
}
