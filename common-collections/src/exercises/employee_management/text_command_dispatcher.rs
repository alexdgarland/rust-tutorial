use crate::exercises::employee_management::text_command_executor::TextCommandExecutor;

struct TextCommandDispatcher {
    command_executors: Vec<Box<dyn TextCommandExecutor>>
}

impl TextCommandDispatcher {
    pub fn process(&mut self, text_command: String) -> Result<(), String> {
        Ok(())
        // TODO - Implement and test this method
        // PSEUDOCODE:
        // For each executor:
        //      Run
        //      If returns Ok(()), exit returning Ok(())
        //      Else iterate to next executor
        // If exhaust list without getting an Ok(()) result:
        //      Err(String::from("No match could be found for submitted text command"))
    }
}

#[cfg(test)]
mod text_command_tests {
    use super::TextCommandDispatcher;
}
