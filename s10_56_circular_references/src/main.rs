use std::rc::Rc;
use std::cell::RefCell;

// student* --- *course

//
// The real solution is to use a third concept, enrollment
// that completely circumvent the need of circular references.
//
// students
// course
// Vec<Enrollment{ course, student }<
//

struct StudentNaive1<'a> {
    name: String,
    courses: Vec<&'a CourseNaive1<'a>>
}

impl <'a> StudentNaive1<'a>
{
    fn new(name: &str) -> StudentNaive1<'a>
    {
        StudentNaive1 {
            name: name.into(),
            courses: Vec::new()
        }
    }
}

struct CourseNaive1<'a>
{
    name: String,
    students: Vec<&'a StudentNaive1<'a>>
}

impl<'a> CourseNaive1<'a>
{
    fn new(name: &str) -> CourseNaive1<'a>
    {
        CourseNaive1 {
            name: name.into(),
            students: Vec::new()
        }
    }

    fn add_students(&'a mut self,
        student: &'a mut StudentNaive1<'a>)
    {
        student.courses.push(self);
        // In this naive implementation, rustc fails with the following error:
        // "cannot borrow `self.students` as mutable because it is also borrowed
        // as immutable". This part could be fixed using a `RefCell`. However,
        // we still have to lifetime issue (see `main`).
        // self.students.push(student);
    }
}

fn circular_ref_naive_1()
{
    let mut john_naive = StudentNaive1::new("John");
    let mut course_naive = CourseNaive1::new("Rust CourseNaive2");
    course_naive.add_students(&mut john_naive);
    // With the naive above version, there is an issue with lifetimes. What
    // will occur when the student get dropped? The course still exists and
    // as a ref to the student. The reverse is also true.

}

struct StudentNaive2 {
    name: String,
    courses: Vec<Rc<RefCell<CourseNaive2>>>
}

impl StudentNaive2
{
    fn new(name: &str) -> StudentNaive2
    {
        StudentNaive2 {
            name: name.into(),
            courses: Vec::new()
        }
    }
}

struct CourseNaive2 {
    name: String,
    students: Vec<Rc<RefCell<StudentNaive2>>>
}

impl CourseNaive2
{
    fn new(name: &str) -> CourseNaive2
    {
        CourseNaive2 {
            name: name.into(),
            students: Vec::new()
        }
    }

    fn add_student(
        course: Rc<RefCell<CourseNaive2>>,
        student: Rc<RefCell<StudentNaive2>>
    ) {
        student.borrow_mut().courses.push(course.clone());
        course.borrow_mut().students.push(student.clone());
    }

}

fn circular_ref_naive_2()
{
    // This seconds approach works. However, you end up completely
    // circumventing the borrow checker by using `RefCells` and
    // the result is quite messy.
    let john = Rc::new(
        RefCell::new(
            StudentNaive2::new("John")
        )
    );

    let jane = Rc::new(
        RefCell::new(
            StudentNaive2::new("Jane")
        )
    );

    let course = CourseNaive2::new("Rust CourseNaive2");
    let magic_course = Rc::new(
        RefCell::new(
            course
        )
    );

    // See here, this `add_student` static method takes immutable instances but
    // mutate these inside. This is not good...
    CourseNaive2::add_student(magic_course.clone(), john);
    CourseNaive2::add_student(magic_course, jane);
}

struct Student {
    name: String
}

impl Student {
    fn courses(&self, platform: Platform)
     -> Vec<String>
    {
        platform.enrollments.iter()
            .filter(|&e| e.student.name == self.name)
            .map(|e| e.course.name.clone())
            .collect()
    }
}

struct Course {
    name: String
}

struct Enrollment<'a> {
    student: &'a Student,
    course: &'a Course
}

impl<'a> Enrollment<'a> {
    fn new(student: &'a Student, course: &'a Course)
     -> Enrollment<'a>
    {
        Enrollment { student, course }
    }
}

struct Platform<'a> {
    enrollments: Vec<Enrollment<'a>>
}

impl<'a> Platform<'a> {
    fn new() -> Platform<'a> {
        Platform {
            enrollments: Vec::new()
        }
    }

    fn enroll(&mut self,
        student: &'a Student,
        course: &'a Course)
    {
        self.enrollments.push(
            Enrollment::new(student, course)
        );
    }
}

fn without_circular_ref()
{
    let john = Student {
        name: "John".into()
    };

    let course = Course {
        name: "Intro to Rust".into()
    };

    let mut p = Platform::new();

    p.enroll(&john, &course);

    for c in john.courses(p) {
        println!("John is taking {}", c)
    }
}

fn main() {
    // So basically, rust allows you to design things with circular
    // references. However, this is most ofter not a good design as
    // it bypasses the borrow checker.
    // Oftentime, best thing to do is to redesign to avoid circular
    // references altogether.
    circular_ref_naive_1();
    circular_ref_naive_2();
    without_circular_ref();
}
