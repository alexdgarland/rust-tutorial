use std::collections::HashMap;

#[derive(Eq, PartialEq, Ord, PartialOrd, Debug)]
struct DepartmentInfo {
    department: String,
    employee_names: Vec<String>,
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

struct EmployeeStore {
    map: HashMap<String, Vec<String>>
}

impl EmployeeStore {
    fn create() -> EmployeeStore {
        let map = HashMap::new();
        EmployeeStore { map }
    }

    fn add_employee(&mut self, employee_name: String, department: String) {
        let department_employees = self.map
            .entry(department)
            .or_insert(vec![]);
        department_employees.push(employee_name);
        department_employees.sort_unstable();
    }

    fn retrieve_employees_by_department(&self, department: String) -> Option<&Vec<String>> {
        self.map.get(&department)
    }

    fn retrieve_all_employees(&self) -> Vec<DepartmentInfo> {
        let mut infos: Vec<DepartmentInfo> = self.map
            .iter()
            .map(DepartmentInfo::create_from_refs)
            .collect();
        infos.sort_unstable();
        infos
    }
}

#[cfg(test)]
mod tests {
    use crate::exercises::employee_store::{EmployeeStore, DepartmentInfo};
    use std::collections::HashMap;

    fn get_department_one() -> String { String::from("Pie Quality Control") }

    fn get_department_two() -> String { String::from("Stealthy Buccaneering") }

    fn get_name_one() -> String { String::from("Bob Bobertson") }

    fn get_name_two() -> String { String::from("Weebl Bull") }

    fn get_name_three() -> String { String::from("Chris the Ninja Pirate") }

    #[test]
    fn test_add_employee_to_new_department() {
        let mut store = EmployeeStore::create();
        store.add_employee(get_name_one(), get_department_one());
        assert_eq!(store.map.get(&get_department_one()), Some(&vec![get_name_one()]));
    }

    #[test]
    fn test_add_employee_to_existing_department() {
        let mut store = EmployeeStore::create();
        store.add_employee(get_name_one(), get_department_one());
        store.add_employee(get_name_two(), get_department_one());
        assert_eq!(
            store.map.get(&get_department_one()),
            Some(&vec![get_name_one(), get_name_two()])
        );
    }

    #[test]
    fn test_add_employee_to_existing_department_maintains_sort_order() {
        let mut store = EmployeeStore::create();
        store.add_employee(get_name_two(), get_department_one());
        store.add_employee(get_name_one(), get_department_one());
        assert_eq!(
            store.map.get(&get_department_one()),
            Some(&vec![get_name_one(), get_name_two()])
        );
    }

    #[test]
    fn test_retrieve_employees_for_missing_department_returns_none() {
        let store = EmployeeStore::create();
        assert_eq!(store.retrieve_employees_by_department(get_department_one()), None);
    }

    #[test]
    fn test_retrieve_employees_for_existing_department_returns_employees() {
        let employees = vec![get_name_one(), get_name_two()];
        let mut map = HashMap::new();
        map.insert(get_department_one(), employees.clone());
        let store = EmployeeStore { map };
        let expected = Some(&employees);
        assert_eq!(store.retrieve_employees_by_department(get_department_one()), expected);
    }

    #[test]
    fn test_retrieve_all_employees_for_new_store_returns_empty_vector() {
        let store = EmployeeStore::create();
        let expected: Vec<DepartmentInfo> = vec![];
        assert_eq!(store.retrieve_all_employees(), expected);
    }

    #[test]
    fn test_retrieve_all_employees_for_populated_store_returns_expected_vector() {
        let mut map = HashMap::new();
        map.insert(get_department_two(), vec![get_name_three()]);
        map.insert(get_department_one(), vec![get_name_one(), get_name_two()]);
        let store = EmployeeStore { map };
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
}
