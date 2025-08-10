use crate::storage::{save_table, append_row, load_table};
use crate::validation::validate_row;

#[derive(Debug, Clone)]
pub enum DataType {
    Text,
    Integer,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Text(String),
    Integer(i64),
}

#[derive(Debug, Clone)]
pub struct Column {
    pub name: String,
    pub data_type: DataType,
}

#[derive(Debug, Clone)]
pub struct Table {
    pub name: String,
    pub columns: Vec<Column>,
}

impl Table {
    pub fn insert(&self, values: Vec<Value>) -> Result<(), String> {
        validate_row(&self.columns, &values)?;
        let row:Vec<String> = values.into_iter().map(|v| match v {
            Value::Integer(i) => i.to_string(),
            Value::Text(s) => s.clone(),
        }).collect();
        append_row(&self.name, row)
    }

    pub fn select_all(&self) -> Result<Vec<Vec<Value>>, String> {
        let raw_rows = load_table(&self.name)?;
        Ok(raw_rows.into_iter().map(|row| {
            row.into_iter().enumerate().map(|(i, raw)| {
                match self.columns[i].data_type {
                    DataType::Integer => Value::Integer(raw.parse().unwrap_or(0)),
                    DataType::Text => Value::Text(raw),
                }
            }).collect()
        }).collect())
    }

    pub fn update(&self, where_column: &str, where_value: Value, updates: Vec<(&str, Value)>) -> Result<(), String> {
        let where_index = self.columns.iter().position(|c| c.name == where_column)
            .ok_or_else(|| format!("Column {} not found", where_column))?;

        let mut rows = self.select_all()?;
        for row in rows.iter_mut() {
            if row[where_index] == where_value {
                for (col_name, new_val) in &updates {
                    if let Some(col_index) = self.columns.iter().position(|c| &c.name == col_name) {
                        row[col_index] = new_val.clone();
                    }
                }
            }
        }

        let string_rows: Vec<Vec<String>> = rows.into_iter().map(|row| {
            row.into_iter().map(|v| match v {
                Value::Integer(i) => i.to_string(),
                Value::Text(s) => s,
            }).collect()
        }).collect();

        save_table(&self.name, string_rows)
    }

    pub fn delete(&self, where_column: &str, where_value: Value) -> Result<(), String> {
        let where_index = self.columns.iter().position(|c| c.name == where_column)
            .ok_or_else(|| format!("Column {} not found", where_column))?;

        let rows = self.select_all()?;
        let filtered: Vec<Vec<String>> = rows.into_iter()
            .filter(|row| row[where_index] != where_value)
            .map(|row| row.into_iter().map(|v| match v {
                Value::Integer(i) => i.to_string(),
                Value::Text(s) => s,
            }).collect())
            .collect();

        save_table(&self.name, filtered)
    }
}
