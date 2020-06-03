mod add_employee;
mod retrieve_all;
mod retrieve_department;

pub trait TextCommandExecutor {
    fn try_execute(&mut self, command: &String) -> Result<(), String>;
}
