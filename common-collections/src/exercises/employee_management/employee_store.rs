use std::collections::HashMap;
use std::fmt::Debug;

use mockall_derive::automock;

// TODO - a lot of the Strings used here for params etc could probably be &str's -
// TODO change here and at site of use (otherwise calling code will auto-coerce to &str and stay verbose)

#[derive(Eq, PartialEq, Ord, PartialOrd, Debug, Clone)]
pub struct DepartmentInfo {
    pub department: String,
    pub employee_names: Vec<String>,
}

impl DepartmentInfo {
    fn create_from_refs(refs: (&String, &Vec<String>)) -> DepartmentInfo {
        let (department_ref, employee_names_ref) = refs;
        DepartmentInfo {
            department: department_ref.clone(),
            employee_names: employee_names_ref.clone(),
        }
    }
}

#[automock]
pub trait EmployeeStore {
    fn add_employee(&mut self, employee_name: &String, department: &String);

    fn retrieve_employees_by_department(&self, department: &String) -> Option<Vec<String>>;

    fn retrieve_all_employees(&self) -> Vec<DepartmentInfo>;

    fn list_departments(&self) -> Vec<String>;
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
        self.map.get(department).map(|names| names.clone())
    }

    fn retrieve_all_employees(&self) -> Vec<DepartmentInfo> {
        let mut infos: Vec<DepartmentInfo> = self.map
            .iter()
            .map(DepartmentInfo::create_from_refs)
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
}

pub(crate) fn setup_mock(setup_behaviour: fn(&mut MockEmployeeStore) -> ()) -> MockEmployeeStore {
    let mut mock_store = MockEmployeeStore::new();
    setup_behaviour(&mut mock_store);
    mock_store
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::{DepartmentInfo, EmployeeStore, EmployeeStoreImpl};

    fn get_department_one() -> String { String::from("Pie Quality Control") }

    fn get_department_two() -> String { String::from("Stealthy Buccaneering") }

    fn get_name_one() -> String { String::from("Bob Bobertson") }

    fn get_name_two() -> String { String::from("Weebl Bull") }

    fn get_name_three() -> String { String::from("Chris the Ninja Pirate") }

    #[test]
    fn test_add_employee_to_new_department() {
        let mut store = EmployeeStoreImpl::new();
        store.add_employee(&get_name_one(), &get_department_one());
        assert_eq!(store.map.get(&get_department_one()), Some(&vec![get_name_one()]));
    }

    #[test]
    fn test_add_employee_to_existing_department() {
        let mut store = EmployeeStoreImpl::new();
        store.add_employee(&get_name_one(), &get_department_one());
        store.add_employee(&get_name_two(), &get_department_one());
        assert_eq!(
            store.map.get(&get_department_one()),
            Some(&vec![get_name_one(), get_name_two()])
        );
    }

    #[test]
    fn test_add_employee_to_existing_department_maintains_sort_order() {
        let mut store = EmployeeStoreImpl::new();
        store.add_employee(&get_name_two(), &get_department_one());
        store.add_employee(&get_name_one(), &get_department_one());
        assert_eq!(
            store.map.get(&get_department_one()),
            Some(&vec![get_name_one(), get_name_two()])
        );
    }

    #[test]
    fn test_retrieve_employees_for_missing_department_returns_none() {
        let store = EmployeeStoreImpl::new();
        assert_eq!(store.retrieve_employees_by_department(&get_department_one()), None);
    }

    #[test]
    fn test_retrieve_employees_for_existing_department_returns_employees() {
        let employees = vec![get_name_one(), get_name_two()];
        let mut map = HashMap::new();
        map.insert(get_department_one(), employees.clone());
        let store = EmployeeStoreImpl { map };
        let expected = Some(employees);
        assert_eq!(store.retrieve_employees_by_department(&get_department_one()), expected);
    }

    #[test]
    fn test_retrieve_all_employees_for_new_store_returns_empty_vector() {
        let store = EmployeeStoreImpl::new();
        let expected: Vec<DepartmentInfo> = vec![];
        assert_eq!(store.retrieve_all_employees(), expected);
    }

    #[test]
    fn test_retrieve_all_employees_for_populated_store_returns_expected_vector() {
        let mut map = HashMap::new();
        map.insert(get_department_two(), vec![get_name_three()]);
        map.insert(get_department_one(), vec![get_name_one(), get_name_two()]);
        let store = EmployeeStoreImpl { map };
        let expected = vec![
            DepartmentInfo {
                department: get_department_one(),
                employee_names: vec![get_name_one(), get_name_two()],
            },
            DepartmentInfo {
                department: get_department_two(),
                employee_names: vec![get_name_three()],
            }
        ];
        assert_eq!(store.retrieve_all_employees(), expected);
    }

    #[test]
    fn test_list_departments_empty() {
        let store = EmployeeStoreImpl::new();
        let expected: Vec<String> = vec![];
        assert_eq!(store.list_departments(), expected);
    }

    #[test]
    fn test_list_departments_populated() {
        let mut map = HashMap::new();
        map.insert("Department A".to_string(), vec!["Employee 1".to_string(), "Employee 2".to_string()]);
        map.insert("Department B".to_string(), vec!["Employee 3".to_string(), "Employee 4".to_string()]);
        let store = EmployeeStoreImpl{map} ;
        let expected: Vec<String> = vec!["Department A".to_string(), "Department B".to_string()];
        assert_eq!(store.list_departments(), expected);
    }
}