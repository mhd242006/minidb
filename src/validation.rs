use crate::schema::{Value, DataType, Column};

pub fn validate_row(columns: &Vec<Column>, values: &Vec<Value>) -> Result<(), String> {
    if columns.len() != values.len() {
        return Err(format!("Invalid number of columns: {}", columns.len()));
    }
    for (i, value) in values.iter().enumerate() {
        let col_type = &columns[i].data_type;
        match (col_type, value) {
            (DataType::Integer, Value::Integer(_)) => {}
            (DataType::Text, Value::Text(_)) => {}
            _ => {
                return Err(format!("Type mismatch for column '{}'", columns[i].name));
            }
        }
    }
    Ok(())
}

