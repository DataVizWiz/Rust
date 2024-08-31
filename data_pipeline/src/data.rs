pub enum DataValue {
    Integer(i32),
    Float(f32),
    Text(String),
    Boolean(bool)
}

pub struct Metadata {
    pub source: String,
    pub timestamp: String,
    pub tags: Vec<String>
}

pub struct DataEntry {
    pub id: i32,
    pub value: DataValue,
    pub metadata: Metadata
}

impl DataEntry {
    pub fn new(
        id: i32, 
        value: DataValue, 
        source: &str,
        timestamp: &str,
        tags: Vec<String>
    ) -> DataEntry {
        DataEntry {
            id,
            value,
            metadata: Metadata {
                source: source.to_string(),
                timestamp: timestamp.to_string(),
                tags
            }
        }
    }

    pub fn display(&self) {
        let value_str = match &self.value {
            DataValue::Integer(x) => format!("{}", x),
            DataValue::Float(x) => format!("{}", x),
            DataValue::Text(x) => format!("{}", x),
            DataValue::Boolean(x) => format!("{}", x),
        };
        
        println!(
            "ID: {}, Value: {}, Source: {}, Timestamp: {}, Tags: {:?}", 
            self.id, 
            value_str, 
            self.metadata.source,
            self.metadata.timestamp,
            self.metadata.tags
        )
    }
}