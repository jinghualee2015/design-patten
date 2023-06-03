use department::{Cashier, Doctor, Medical, Reception};
use patient::Patient;

use crate::department::Department;

mod patient;
mod department;

fn main() {
    // println!("Hello, world!");
    let cashier = Cashier::default();
    let medical = Medical::new(cashier);
    let doctor = Doctor::new(medical);

    let mut reception = Reception::new(doctor);

    let mut patient = Patient {
        name: "John".into(),
        ..Patient::default()
    };

    reception.execute(&mut patient);
    println!("\nThe patient has been already handled:\n");

    reception.execute(&mut patient);
}
