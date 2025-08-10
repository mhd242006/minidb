use crate::schema::{Table, Value};

pub fn execute_query(query: &str, table: &mut Table) -> Result<(), String>{
    let parts: Vec<&str> = query.trim().split_whitespace().collect();
    match parts[0].to_uppercase().as_str() {
        "INSERT" => {
            let values: Vec<Value> = parts[1..].iter().map(|v| {
                if let Ok(i) = v.parse::<i64>() {
                    Value::Integer(i)
                } else {
                    Value::Text(v.to_string())
                }
            }).collect();
            table.insert(values)
        }
        _ => Err("Unknown query".to_string()),
    }
}