use std::collections::HashMap;
use std::fmt::Debug;

use mockall_derive::automock;

#[derive(Eq, PartialEq, Ord, PartialOrd, Debug, Clone)]
pub struct DepartmentInfo {
    pub department: String,
    pub employee_names: Vec<String>,
}

#[automock]
pub trait EmployeeStore {
    fn add_employee(&mut self, employee_name: &String, department: &String);

    fn retrieve_employees_by_department(&self, department: &String) -> Option<Vec<String>>;

    fn retrieve_all_employees(&self) -> Vec<DepartmentInfo>;

    fn list_departments(&self) -> Vec<String>;

    fn delete_department(&mut self, department: &String) -> Result<DepartmentInfo, String>;
}

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
                Err(department.clone())
            },
            Some(employee_names) => {
                let deleted_department = DepartmentInfo {
                    department: department.clone(),
                    employee_names: employee_names.clone()
                };
                self.map.remove(department);
                Ok(deleted_department)
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::{DepartmentInfo, EmployeeStore, EmployeeStoreImpl};

    fn department_one() -> String { String::from("Pie Quality Control") }
    fn department_two() -> String { String::from("Stealthy Buccaneering") }
    fn name_one() -> String { String::from("Bob Bobertson") }
    fn name_two() -> String { String::from("Weebl Bull") }
    fn name_three() -> String { String::from("Chris the Ninja Pirate") }
    fn deptone_names() -> Vec<String> { vec![name_one(), name_two()] }
    fn depttwo_names() -> Vec<String> { vec![name_three()] }

    fn populated_store() -> EmployeeStoreImpl {
        let mut map = HashMap::new();
        map.insert(department_two(), vec![name_three()]);
        map.insert(department_one(), vec![name_one(), name_two()]);
        let store = EmployeeStoreImpl { map };
        store
    }

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
            DepartmentInfo { department: department_one(), employee_names: deptone_names(), },
            DepartmentInfo { department: department_two(), employee_names: depttwo_names(), },
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

        let expected_return = Ok(DepartmentInfo { department: department_one(), employee_names: deptone_names()} );
        assert_eq!(actual_return, expected_return);
        let mut expected_map = HashMap::new();
        expected_map.insert(department_two(), depttwo_names());
        assert_eq!(store.map, expected_map);
    }

    #[test]
    fn test_delete_non_existent_department() {
        let mut store = EmployeeStoreImpl::new();
        let actual_return = store.delete_department(&department_one());
        assert_eq!(actual_return, Err(department_one()));
    }
}