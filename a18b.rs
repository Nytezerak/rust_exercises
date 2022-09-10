// Topic: Result & the question mark operator
//
// Requirements:
// * Determine if an employee can access a building using a digital keycard
// * Employees that can access the building are:
//   * Maintenance crews
//   * Marketing department employees
//   * Manager
// * Other employees that work at the company are:
//   * Line supervisors
//   * Kitchen staff
//   * Assembly technicians
// * Ensure that terminated employees cannot access the building
//   regardless of their position
//
// Notes:
// * Use an enum to represent all types of employees
// * Use a struct to store the employee type and whether they are
//   still employed
// * Use a function that returns a Result to determine if the employee
//   may enter the building
// * Print whether the employee may access the building
//   * Must use a function that utilizes the question mark operator to do this

enum Position{
    _MaintenceCrew,
    _MarketingDept,
    Manager,
    _LineSupervisor,
    _KitchenStaff,
    _AssemblyTechs,
}

enum Status{
    Active,
    _Terminated,
}

struct Employee{
    position: Position,
    status: Status,
}

fn check_access(employee: &Employee)-> Result<(), String>{
    match employee.status {
        Status::_Terminated => return Err("_Terminated!".to_owned()),
        _ => (),
    }
    match employee.position {
        Position::_MaintenceCrew => Ok(()),
        Position::_MarketingDept => Ok(()),
        Position::Manager => Ok(()),
        _ => Err("Access denied".to_owned()),
    }
}

fn print_access(employee: &Employee) -> Result<(), String>{
    let attempt_access = check_access(employee)?;
    println!("Welcome!");
    Ok(())
}
fn main() {
    let employee_junin = Employee{
        position: Position::Manager,
        status: Status::Active,
    };

    match print_access(&employee_junin) {
        Err(e) => println!("Access denied: {:?}", e),
        _ => ()
    }
}
