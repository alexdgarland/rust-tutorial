mod model;

use model::{Spreadsheet, SpreadsheetRow, SpreadsheetCell};

fn show_sheet(description: &str, sheet: &Spreadsheet) {
    info!("Showing contents of {} spreadsheet model", description);
    println!("{}", sheet);
}

fn add_one(cell: &SpreadsheetCell) -> SpreadsheetCell {
    match cell {
        SpreadsheetCell::Int(int) => SpreadsheetCell::Int(int + 1),
        SpreadsheetCell::Float(float) => SpreadsheetCell::Float(float + 1.0),
        SpreadsheetCell::Text(text) => SpreadsheetCell::Text(format!("{} + 1", text))
    }
}


pub fn show_spreadsheet_operations() {
    let sheet = Spreadsheet {
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

    let mapped_sheet = sheet.map(|row| row.map(add_one));

    show_sheet("mapped", &mapped_sheet);
}
