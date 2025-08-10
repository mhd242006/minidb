mod storage;
mod schema;
mod validation;
mod query;

use schema::{Table, Column, DataType, Value};
use std::io::{self, Write};

fn main() -> Result<(), String> {
    let columns = vec![
        Column { name: "id".to_string(), data_type: DataType::Integer },
        Column { name: "name".to_string(), data_type: DataType::Text },
    ];

    let mut table = Table {
        name: "users".to_string(),
        columns,
    };

    println!("Welcome to MiniDB! Type 'exit' to quit.");

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let trimmed = input.trim();

        if trimmed.eq_ignore_ascii_case("exit") {
            break;
        }

        let parts: Vec<&str> = trimmed.split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }

        match parts[0].to_uppercase().as_str() {
            "INSERT" => {
                if parts.len() < 3 {
                    println!("Usage: INSERT <id> <name>");
                    continue;
                }
                let id = match parts[1].parse::<i64>() {
                    Ok(n) => n,
                    Err(_) => {
                        println!("Invalid id");
                        continue;
                    }
                };
                let name = parts[2].to_string();

                match table.insert(vec![Value::Integer(id), Value::Text(name)]) {
                    Ok(_) => println!("Inserted."),
                    Err(e) => println!("Error: {}", e),
                }
            }

            "SELECT" => {
                match table.select_all() {
                    Ok(rows) => {
                        for row in rows {
                            println!("{:?}", row);
                        }
                    }
                    Err(e) => println!("Error: {}", e),
                }
            }

            "UPDATE" => {
                if parts.len() < 5 {
                    println!("Usage: UPDATE <where_column> <where_value> <update_column> <update_value>");
                    continue;
                }

                let where_column = parts[1];
                let where_value = parse_value(parts[2])?;
                let update_column = parts[3];
                let update_value = parse_value(parts[4])?;

                match table.update(where_column, where_value, vec![(update_column, update_value)]) {
                    Ok(_) => println!("Updated."),
                    Err(e) => println!("Error: {}", e),
                }
            }

            "DELETE" => {
                if parts.len() < 3 {
                    println!("Usage: DELETE <where_column> <where_value>");
                    continue;
                }
                let where_column = parts[1];
                let where_value = parse_value(parts[2])?;

                match table.delete(where_column, where_value) {
                    Ok(_) => println!("Deleted."),
                    Err(e) => println!("Error: {}", e),
                }
            }

            _ => {
                println!("Unknown command");
            }
        }
    }

    println!("Goodbye!");
    Ok(())
}

fn parse_value(s: &str) -> Result<Value, String> {
    if let Ok(n) = s.parse::<i64>() {
        Ok(Value::Integer(n))
    } else {
        Ok(Value::Text(s.to_string()))
    }
}
