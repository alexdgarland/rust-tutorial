use std::fmt;
use std::fmt::Formatter;

enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String)
}

impl fmt::Display for SpreadsheetCell {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let value: String = match self {
            SpreadsheetCell::Int(int) => int.to_string(),
            SpreadsheetCell::Float(float) => float.to_string(),
            SpreadsheetCell::Text(text) => text.to_string()
        };
        write!(f, "{}", value)
    }
}

struct SpreadsheetRow {
    cells: Vec<SpreadsheetCell>
}

impl fmt::Display for SpreadsheetRow {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let cells_as_strings: Vec<String> = self.cells
            .iter()
            .map(|el| format!(" {} ", el))
            .collect();
        write!(f, "|{}|", cells_as_strings.join("|"))
    }
}

struct Spreadsheet {
    rows: Vec<SpreadsheetRow>
}

impl fmt::Display for Spreadsheet {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let rows_as_string: Vec<String> = self.rows
            .iter()
            .map(|el| el.to_string())
            .collect();
        write!(f, "{}", rows_as_string.join("\n"))
    }
}

fn show_sheet(description: &str, sheet: &Spreadsheet) {
    println!("*** Showing full contents of {} spreadsheet model ***\n", description);
    println!("{}\n", sheet);
}

pub fn show_spreadsheet_operations() {

    let mut sheet = Spreadsheet {
        rows: vec![
            SpreadsheetRow {
                cells: vec![
                    SpreadsheetCell::Int(1),
                    SpreadsheetCell::Float(10.99),
                    SpreadsheetCell::Text(String::from("Hello World!"))
                ]
            },
            SpreadsheetRow {
                cells: vec![
                    SpreadsheetCell::Int(2),
                    SpreadsheetCell::Float(11.99),
                    SpreadsheetCell::Text(String::from("Hello again!"))
                ]
            },
            SpreadsheetRow {
                cells: vec![
                    SpreadsheetCell::Int(3),
                    SpreadsheetCell::Float(12.99),
                    SpreadsheetCell::Text(String::from("Oh hi... still here?"))
                ]
            }
        ]
    };

    show_sheet("initial", &sheet);

    // TODO - create a match function which updates any types of cell by adding one
    // Int -> +1
    // Float -> +1.0
    // Text -> format!("{} +1", existing_string_value)
    sheet.rows[0].cells[0] = SpreadsheetCell::Int(100);
    show_sheet("hackily modified", &sheet);
}
