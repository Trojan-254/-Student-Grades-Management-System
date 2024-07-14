use::std::collections::HashMap;
use::std::io;

fn main() {
    //Entry to the main function

    //A hash map to capture the students details
    let mut grades: HashMap<String, f32> = HashMap::new();

    //A loop to get user input
    loop {
        println!("=============== Welcome to the Student Grade Management System ===============");
        println!("           ");
        println!("====== Please choose a number ======");
        println!("           ");
        println!("1. Add student");
        println!("2. View a student's grade");
        println!("3. Update a student's grade");
        println!("4. Delete student");
        println!("5. List all students");
        println!("6. Exit");

        let mut choice = String::new(); //Declared variable to store the user's choice
        io::stdin().read_line(&mut choice).expect("Failed to readline"); // Get's the choice
        let choice: u32 = choice.trim().parse().expect("Please choose one of the numbers listed");

        // We now match the choice to the requirements
        match choice {
            1 => add_student(&mut grades),
            2 => view_grades(&grades),
            3 => update_grade(&mut grades),
            4 => delete_student(&mut grades),
            5 => list_students(&grades),
            6 => break,
            _ => println!("Invalid choice! Please try again"),
        }
    }
}

// Function to add student
fn add_student(grades: &mut HashMap<String, f32>) {
    let (name, grade) = get_student_info();
    grades.insert(name, grade);
    println!("🎉 Student has been added succesfully 🎉");
}

//Function to view the student's grades
fn view_grades(grades: &HashMap<String, f32>) {
    let name = get_student_name();
    match grades.get(&name) {
        Some(grade) => println!("Grade: {}", grade),
        None => println!("😔 Student not found! 😔"),
    }
}

// Fuction to update student grade
fn update_grade(grades: &mut HashMap<String, f32>) {
    let name = get_student_name();
    if grades.contains_key(&name) {
        let (new_name, new_grade) = get_student_info();
        grades.insert(new_name, new_grade);
        println!("🎉 Students grade updated succesfully!🎉");
    } else {
        println!("😔 Student not found! 😔");
    }
}

// Function to list all students
fn list_students(grades: &HashMap<String, f32>) {
    for (name, grade) in grades {
        println!("{}: {}", name, grade);
    }
}

// Function to delete student
fn delete_student(grades: &mut HashMap<String, f32>) {
    let name = get_student_name();
    if grades.contains_key(&name) {
        grades.remove(&name).is_some();
        println!("Warning this operation is irreversable");
        println!("🎉 Student has been deleted succesfully! 🎉");
    } else {
        println!("Student not found");
    }
}

// Function to get the student's name
fn get_student_name() -> String {
    let mut name = String::new();
    println!("Please enter the student's name!...");
    io::stdin().read_line(&mut name).expect("Failed to readline");
    let student = name.trim().to_string();
    return student;
}

// Function to get the students info
fn get_student_info() -> (String, f32) {
    let mut name = get_student_name();
    let mut grade = String::new();
    println!("Please enter the student's grade!...");
    io::stdin().read_line(&mut grade).expect("Failed to readline!");
    (name, grade.trim().parse().expect("Please enter a valid grade"))
}
