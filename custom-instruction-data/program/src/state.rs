use borsh::{BorshDeserialize, BorshSerialize};

#[derive(Debug, BorshDeserialize, BorshSerialize)]
pub enum ProgramInstruction {
    CreateUser(User),
    CreateStudent(Student),
}

#[derive(Debug, BorshDeserialize, BorshSerialize)]
pub struct User {
    pub name: String,
    pub age: u8,
}
#[derive(Debug, BorshDeserialize, BorshSerialize)]
pub struct Student {
    pub student_id: String,
    pub university: University,
    pub grades: [u8; 5],
}

#[derive(Debug, BorshDeserialize, BorshSerialize)]
pub enum University {
    UniversityOfToronto,
    QueensUniversity,
    YorkUniversity,
    WesternUniversity,
}
