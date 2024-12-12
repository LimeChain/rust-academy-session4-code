pub trait Name {
    const NAME: &str;

    fn name() -> String;
}

pub struct Banica;

impl Name for Banica {
    const NAME: &str = "Banica sys sirene";

    fn name() -> String {
        String::from("Banica sys sirene")
    }
}

pub fn proba() {
    let _ = Banica::name(); // run-time
    let _ = Banica::NAME; // compile-time
}
