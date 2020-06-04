use std::fmt::{Formatter, Display, Result};

fn display_elements<T: Display>(f: &mut Formatter<'_>, elements: &Vec<T>, sep: &str) -> Result {
    let elems_as_strings: Vec<String> = elements
        .iter()
        .map(|e| format!("{}", e))
        .collect();
    write!(f, "{}{}{}", sep, elems_as_strings.join(sep), sep)
}

pub enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

impl Display for SpreadsheetCell {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let value: String = match self {
            SpreadsheetCell::Int(int) => int.to_string(),
            SpreadsheetCell::Float(float) => float.to_string(),
            SpreadsheetCell::Text(text) => text.to_string()
        };
        write!(f, "{}", value)
    }
}

pub struct SpreadsheetRow {
    pub cells: Vec<SpreadsheetCell>
}

impl Display for SpreadsheetRow {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        display_elements(f, &self.cells, " | ")
    }
}

impl SpreadsheetRow {
    pub fn map(&self, f: fn(&SpreadsheetCell) -> SpreadsheetCell) -> SpreadsheetRow {
        SpreadsheetRow { cells: self.cells.iter().map(f).collect() }
    }
}

pub struct Spreadsheet {
    pub rows: Vec<SpreadsheetRow>
}

impl Display for Spreadsheet {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        display_elements(f, &self.rows, "\n")
    }
}

impl Spreadsheet {
    pub fn map(&self, f: fn(&SpreadsheetRow) -> SpreadsheetRow) -> Spreadsheet {
        Spreadsheet { rows: self.rows.iter().map(f).collect() }
    }
}