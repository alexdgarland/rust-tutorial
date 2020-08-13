use std::collections::HashMap;

use super::{DepartmentInfo, EmployeeDeletionResult, EmployeeStore};
use super::EmployeeDeletionResult::{EmployeeNotInDepartment, NoSuchDepartment, SuccessfullyDeleted};

#[derive(Debug, PartialEq, Eq)]
pub struct EmployeeStoreImpl {
    map: HashMap<String, Vec<String>>
}

impl EmployeeStoreImpl {
    pub fn new() -> EmployeeStoreImpl {
        EmployeeStoreImpl { map: HashMap::new() }
    }
}

impl EmployeeStore for EmployeeStoreImpl {
    fn add_employee(&mut self, employee_name: &String, department: &String) {
        let department_employees = self.map
            .entry(department.clone())
            .or_insert(vec![]);
        department_employees.push(employee_name.clone());
        department_employees.sort_unstable();
    }

    fn retrieve_employees_by_department(&self, department: &String) -> Option<Vec<String>> {
        self.map
            .get(department)
            .map(|names| names.clone())
    }

    fn retrieve_all_employees(&self) -> Vec<DepartmentInfo> {
        let mut infos: Vec<DepartmentInfo> = self.map
            .iter()
            .map(|(dep, names)|
                DepartmentInfo { department: dep.clone(), employee_names: names.clone() }
            )
            .collect();
        infos.sort_unstable();
        infos
    }

    fn list_departments(&self) -> Vec<String> {
        let mut departments: Vec<String> = self.map
            .keys()
            .map(|d| d.clone())
            .collect();
        departments.sort_unstable();
        departments
    }

    fn delete_department(&mut self, department: &String) -> Result<DepartmentInfo, String> {
        match self.map.get(department) {
            None => {
                Err(format!("Could not delete department \"{}\" - no such department", department))
            }
            Some(employee_names) => {
                let deleted_department = DepartmentInfo {
                    department: department.clone(),
                    employee_names: employee_names.clone(),
                };
                self.map.remove(department);
                Ok(deleted_department)
            }
        }
    }

    fn delete_employee(&mut self, employee_name: &String, department: &String) -> EmployeeDeletionResult {
        match self.map.get_mut(department) {
            None => NoSuchDepartment,
            Some(names_list) => {
                match names_list.iter().position(|en| en == employee_name)
                {
                    None => EmployeeNotInDepartment,
                    Some(index) => {
                        names_list.remove(index);
                        SuccessfullyDeleted
                    }
                }
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::employee_store::EmployeeDeletionResult::{
        EmployeeNotInDepartment, NoSuchDepartment, SuccessfullyDeleted,
    };

    use super::{DepartmentInfo, EmployeeStore, EmployeeStoreImpl};

    fn department_one() -> String { String::from("Pie Quality Control") }

    fn department_two() -> String { String::from("Stealthy Buccaneering") }

    fn name_one() -> String { String::from("Bob Bobertson") }

    fn name_two() -> String { String::from("Weebl Bull") }

    fn name_three() -> String { String::from("Chris the Ninja Pirate") }

    fn deptone_names() -> Vec<String> { vec![name_one(), name_two()] }

    fn depttwo_names() -> Vec<String> { vec![name_three()] }

    fn initial_populated_map() -> HashMap<String, Vec<String>> {
        let mut map = HashMap::new();
        map.insert(department_two(), vec![name_three()]);
        map.insert(department_one(), vec![name_one(), name_two()]);
        map
    }

    fn populated_store() -> EmployeeStoreImpl {
        EmployeeStoreImpl { map: initial_populated_map() }
    }

    fn non_existent_employee() -> String { String::from("Hairy Lee") }

    fn non_existent_department() -> String { String::from("Pie Rejection") }

    #[test]
    fn test_add_employee_to_new_department() {
        let mut store = EmployeeStoreImpl::new();
        store.add_employee(&name_one(), &department_one());
        assert_eq!(store.map.get(&department_one()), Some(&vec![name_one()]));
    }

    #[test]
    fn test_add_employee_to_existing_department() {
        let mut store = EmployeeStoreImpl::new();
        store.add_employee(&name_one(), &department_one());
        store.add_employee(&name_two(), &department_one());
        assert_eq!(
            store.map.get(&department_one()),
            Some(&vec![name_one(), name_two()])
        );
    }

    #[test]
    fn test_add_employee_to_existing_department_maintains_sort_order() {
        let mut store = EmployeeStoreImpl::new();
        store.add_employee(&name_two(), &department_one());
        store.add_employee(&name_one(), &department_one());
        assert_eq!(
            store.map.get(&department_one()),
            Some(&vec![name_one(), name_two()])
        );
    }

    #[test]
    fn test_retrieve_employees_for_missing_department_returns_none() {
        assert_eq!(
            EmployeeStoreImpl::new().retrieve_employees_by_department(&department_one()),
            None
        );
    }

    #[test]
    fn test_retrieve_employees_for_existing_department_returns_employees() {
        assert_eq!(
            populated_store().retrieve_employees_by_department(&department_one()),
            Some(deptone_names())
        );
    }

    #[test]
    fn test_retrieve_all_employees_for_new_store_returns_empty_vector() {
        let expected: Vec<DepartmentInfo> = vec![];
        assert_eq!(EmployeeStoreImpl::new().retrieve_all_employees(), expected);
    }

    #[test]
    fn test_retrieve_all_employees_for_populated_store_returns_expected_vector() {
        let expected = vec![
            DepartmentInfo { department: department_one(), employee_names: deptone_names() },
            DepartmentInfo { department: department_two(), employee_names: depttwo_names() },
        ];
        assert_eq!(populated_store().retrieve_all_employees(), expected);
    }

    #[test]
    fn test_list_departments_empty() {
        let expected: Vec<String> = vec![];
        assert_eq!(EmployeeStoreImpl::new().list_departments(), expected);
    }

    #[test]
    fn test_list_departments_populated() {
        assert_eq!(
            populated_store().list_departments(),
            vec![department_one(), department_two()]
        );
    }

    #[test]
    fn test_delete_existing_department() {
        let mut store = populated_store();
        let actual_return = store.delete_department(&department_one());

        let expected_return = Ok(DepartmentInfo { department: department_one(), employee_names: deptone_names() });
        assert_eq!(actual_return, expected_return);
        let mut expected_map = HashMap::new();
        expected_map.insert(department_two(), depttwo_names());
        assert_eq!(store.map, expected_map);
    }

    #[test]
    fn test_delete_non_existent_department() {
        let mut store = EmployeeStoreImpl::new();
        let actual_return = store.delete_department(&department_one());
        let expected_return = Err("Could not delete department \"Pie Quality Control\" - no such department".to_string());
        assert_eq!(actual_return, expected_return);
    }

    #[test]
    fn test_delete_existing_employee() {
        let mut store = populated_store();
        let result = store.delete_employee(&name_one(), &department_one());
        assert_eq!(result, SuccessfullyDeleted);
        assert_eq!(store.retrieve_employees_by_department(&department_one()), Some(vec![name_two()]));
    }

    fn assert_unchanged(populated_store: EmployeeStoreImpl) {
        assert_eq!(populated_store.map, initial_populated_map());
    }

    #[test]
    fn test_fails_to_delete_non_existent_employee() {
        let mut store = populated_store();
        let result = store.delete_employee(
            &non_existent_employee(), &department_one(),
        );
        assert_eq!(result, EmployeeNotInDepartment);
        assert_unchanged(store);
    }

    #[test]
    fn test_fails_to_delete_employee_in_wrong_department() {
        let mut store = populated_store();
        let result = store.delete_employee(
            &name_three(), &department_one(),
        );
        assert_eq!(result, EmployeeNotInDepartment);
        assert_unchanged(store);
    }

    #[test]
    fn test_fails_to_delete_employee_in_non_existent_department() {
        let mut store = populated_store();
        let result = store.delete_employee(
            &name_one(), &non_existent_department(),
        );
        assert_eq!(result, NoSuchDepartment);
        assert_unchanged(store);
    }
}
