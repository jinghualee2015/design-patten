use crate::adaptee::SpecificTarget;
use crate::adapter::TargetAdapter;
use crate::target::{OrdinaryTarget, Target};

mod target;
mod adaptee;
mod adapter;
fn call(target: impl Target) {
    println!("'{}'", target.request());
}
fn main() {
    let target = OrdinaryTarget;

    print!("A compatible target can be directly called: ");
    call(target);

    let adaptee = SpecificTarget;

    println!(
        "Adaptee is incompatible with client: '{}'",
        adaptee.specific_request()
    );

    let adapter = TargetAdapter::new(adaptee);

    print!("But with adapter client can call its method: ");
    call(adapter);
}
