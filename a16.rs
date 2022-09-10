// Topic: Option
//
// Requirements:
// * Print out the details of a student's locker assignment
// * Lockers use numbers and are optional for students
//
// Notes:
// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>
struct StudentLocker{
    name: String,
    locker: Option<i32>,
}

fn main() {
    let student = StudentLocker{
        name: "Cleitin".to_owned(),
        locker: Some(404),
    };

    match student.locker{
        Some(lckr) => println!("Student {:?} uses the locker: {:?}", student.name, lckr),
        None => println!("Student {:?} does not use a locker", student.name)
    }
}
