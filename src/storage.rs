use std::fs;
use std::fs::OpenOptions;
use std::io::Read;
use std::io::Write;
pub fn save_table(table_name: &str, rows: Vec<Vec<String>>) -> Result<(), String> {
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(format!("{}.tbl", table_name))
        .map_err(|e| e.to_string())?;

    for row in rows {
        let line = row.join("|") + "\n";
        file.write_all(line.as_bytes()).map_err(|e| e.to_string())?;
    }
    Ok(())
}

pub fn append_row(table_name: &str, row: Vec<String>) -> Result<(), String> {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(format!("{}.tbl", table_name))
        .map_err(|e| e.to_string())?;

    let line = row.join("|") + "\n";
    file.write_all(line.as_bytes()).map_err(|e| e.to_string())?;

    Ok(())
}

pub fn load_table(table_name: &str) -> Result<Vec<Vec<String>>, String> {
    let contents = fs::read_to_string(format!("{}.tbl", table_name))
        .map_err(|e| e.to_string())?;
    Ok(contents
        .lines()
        .map(|line| line.split('|').map(|v| v.to_string()).collect())
        .collect())
}
