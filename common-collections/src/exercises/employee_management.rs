mod employee_store;
mod text_command_dispatcher;
mod text_command_executor;

use crate::exercises::employee_management::employee_store::create_employee_store;
use text_command_executor::{
    AddEmployeeTextCommandExecutor, RetrieveAllTextCommandExecutor, RetrieveDepartmentTextCommandExecutor,
    TextCommandExecutor
};

pub fn demo_employee_management() {

    let mut employee_store = employee_store::create_employee_store();

    let executors: Vec<Box<dyn TextCommandExecutor>> = vec![
        Box::new(AddEmployeeTextCommandExecutor { employee_store: &mut employee_store }),
        Box::new(RetrieveAllTextCommandExecutor { employee_store: &mut employee_store }),
    ];

    //  TODO - I'm checking in this non-working commit specifically because it is a great example
    //  TODO   of the sort of thing that is deliberately NOT allowed in Rust (after which I should fix it, details follow)
    //  It produces an error like:
    //     error[E0499]: cannot borrow `employee_store` as mutable more than once at a time
    //         --> src/exercises/employee_management.rs:19:67
    //         |
    //         17 |       let executors: Vec<Box<dyn TextCommandExecutor>> = vec![
    //            |  ________________________________________________________-
    //         18 | |         Box::new(AddEmployeeTextCommandExecutor { employee_store: &mut employee_store }),
    //            | |                                                                   ------------------- first mutable borrow occurs here
    //         19 | |         Box::new(RetrieveAllTextCommandExecutor { employee_store: &mut employee_store }),
    //            | |                                                                   ^^^^^^^^^^^^^^^^^^^ second mutable borrow occurs here
    //         20 | |     ];
    //             | |_____- first borrow later used here

}
