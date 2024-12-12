// Разширена типова система в Rust


// #![no_implicit_prelude]
// #![no_std]

#![allow(unused)]

type Neshto<T> = Vec<(T, T)>;

fn kek() {
    let x: Neshto<i32> = vec![];
}








// 1. Newtype Pattern за type safety
mod newtype_pattern {
    // Wrapper около примитивен тип за type safety
    #[repr(transparent)]
    pub(crate) struct Meters(pub f64);
    pub(crate) struct Kilometers(pub f64);

    impl Meters {
        // Създаване с валидация
        pub(crate) fn new(distance: f64) -> Self {
            // Може да добавим проверка
            if distance < 0.0 {
                panic!("Разстоянието не може да е отрицателно");
            }
            Meters(distance)
        }

        // Метод за конвертиране
        fn to_kilometers(self) -> Kilometers {
            Kilometers(self.0 / 1000.0)
        }
    }

    // Спираме директно сравнение
    impl PartialEq for Meters {
        fn eq(&self, other: &Self) -> bool {
            self.0 == other.0
        }
    }

    // Демонстрира изчисление с type-safe разстояние
    fn calculate_journey() {
        let distance = Meters::new(5000.0);
        // Пречи на случайно смесване на мерни единици
        // let kilometers = distance; // Ще е грешка по време на компилация
        let kilometers = distance.to_kilometers();
    }
}

// 2. Type aliases
mod type_aliases {
    // Създаване на по-смислени имена на типове
    pub(crate) type Coordinates = (f64, f64);
    pub(crate) type Result<T> = std::result::Result<T, String>;

    // Използване във функционални сигнатури
    pub(crate) fn calculate_distance(start: Coordinates, end: Coordinates) -> f64 {
        let (x1, y1) = start;
        let (x2, y2) = end;
        ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt()
    }

    // По-сложен alias с generics
    pub(crate) type BoxedFn<Input, Output> = Box<dyn Fn(Input) -> Output>;
}

enum Never {}

fn kek() {
    let x: Never = todo!();

    let y = match x {
    };
}

// 3. Never Type (!)
mod never_type {
    // Функция, която никога не връща
    fn infinite_loop() -> ! {
        let x = loop {
            println!("Това ще работи безкрайно");
        };
    }

    // Diverging функция в match
    fn process_result(result: Result<i32, String>) -> i32 {
        match result {
            Ok(value) => value,
            Err(e) => panic!("Възникна грешка: {}", e), // Връща !
        }
    }

    // Демонстрация на never type в контекст
    #[allow(clippy::unnecessary_literal_unwrap)]
    fn example_usage() {
        let x: Option<i32> = None;

        // Следното е валидно, защото `panic!` връща !
        let value = x.unwrap_or_else(|| panic!("Няма стойност!"));
    }
}

// 4. Dynamically sized types и Sized trait
mod dynamic_sized_types {
    // Trait обекти са динамично оразмерени
    pub(crate) trait Animal {
        fn make_sound(&self);
    }

    struct Dog;
    struct Cat;

    impl Animal for Dog {
        fn make_sound(&self) {
            println!("Бяу!");
        }
    }

    impl Animal for Cat {
        fn make_sound(&self) {
            println!("Мяу!");
        }
    }

    // Използване на trait обекти (динамично оразмерени)
    pub(crate) fn process_animal(animal: &dyn Animal) {
        animal.make_sound();
    }

    // Експлицитно използване на Sized trait-а
    // T: Sized означава, че типът трябва да има известен размер **по време на компилация**
    pub(crate) fn fixed_size_function<T: Sized>(value: T) {
        // Работи само с типове с известен размер
    }

    // Позволяване на unsized типове
    pub(crate) fn dynamic_size_function<T: ?Sized>(value: &T) {
        // Може да работи както със sized, така и с unsized типове
    }

    // Практически пример за динамично оразмерен тип
    // (вече сме обсъждали плюсовете и минусите)
    pub(crate) fn create_animal_vec() -> Vec<Box<dyn Animal>> {
        vec![Box::new(Dog), Box::new(Cat)]
    }
}

fn main() {
    // Демонстриране на различни разширени type концепции

    // Newtype pattern
    let distance = newtype_pattern::Meters(5000.0);

    // Type aliases
    let start: type_aliases::Coordinates = (0.0, 0.0);
    let end: type_aliases::Coordinates = (3.0, 4.0);

    // Dynamically sized types
    let animals = dynamic_sized_types::create_animal_vec();
    animals.iter().for_each(|animal| animal.make_sound());
}
