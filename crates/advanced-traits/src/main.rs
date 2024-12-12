mod add;
mod names;

use add::{Banica, Add};

pub fn main() {
    let unit: () = ();
    let banica: Banica = Banica { a: 5 };

    let a = unit.add(&banica);
    let b = banica.add(&unit);

    dbg!(a, b);
}
