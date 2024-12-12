#![allow(unused)]

use macros::{bulgar, log_method, make_answer, Neshto};

// 1. Declarative Macros (macro_rules!)

// Прост macro за създаване на вектор
macro_rules! create_vector {
    // Без аргументи: празен вектор
    () => {
        Vec::new()
    };

    ($($elem:expr),* $(,)?) => {{
        let mut temp_vec = Vec::new();
        $(temp_vec.push($elem);)*
        temp_vec
    }};
}

mod opa {
    fn main() {
        let _ = create_vector![1,2,3,4,];
        println!["aloda"];
        println!{"aloda"};
    }
}

// Debug macro за разпечатване на израз и неговата стойност
macro_rules! debug_print {
    ($x:expr) => {
        println!("Expression: {} = {:?}", stringify!($x), $x);
    };
}

// Демонстрационен код
fn main() {
    // Употреба на макроси
    #[allow(clippy::vec_init_then_push)]
    let v = create_vector!(1, 2, 3);
    debug_print!(v);

    // Употреба на процедурни макроси
    make_answer!(); // Създава функция answer()
    println!("Отговорът е: {}", answer());

    // ------

    // Derive macro
    #[derive(Neshto)]
    struct Example {
        #[banica]
        field: i32,
    }

    println!("Специален метод: {}", Example::special_method());

    // Attribute macro
    #[log_method(kakvoto i da e =< for {[]} ASD.,> iasd as E)]
    fn example_function() {
        println!("Вътрешно изпълнение");
    }

    example_function();

    // Attribute macro ръчно
    bulgar!{
        {
            let hello = 5;
        }
        let hello = 5;
    }
}
