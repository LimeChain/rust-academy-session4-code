fn main() {

    struct Kek {
        pole: fn(),
    }

    fn kek() {}

    let neshto: Kek = Kek { pole: kek };

    (neshto.pole)();

    fn apply_operation(func: fn(i32) -> i32, value: i32) -> i32 {
        func(value)
    }

    fn square(x: i32) -> i32 {
        x * x
    }

    fn double(x: i32) -> i32 {
        x * 2
    }

    let result1 = apply_operation(square, 5);
    println!("Square of 5: {}", result1); // Outputs: 25

    let result2 = apply_operation(double, 5);
    println!("Double of 5: {}", result2); // Outputs: 10

    //

    fn create_multiplier(factor: i32) -> Box<dyn Fn(i32) -> i32> {
        let mut banica = 5;
        Box::new(move |x| {
            x * factor
        })
    }

    let double_closure = create_multiplier(2);
    let triple_closure = create_multiplier(3);

    println!("Doubling 7: {}", double_closure(7)); // Outputs: 14
    println!("Tripling 7: {}", triple_closure(7)); // Outputs: 21

    //

    fn counter() -> impl FnMut() -> i32 {
        let mut count = 0;

        // Обсъжали сме го по-рано
        move || {
            count += 1;
            count
        }
    }

    let mut increment = counter();
    println!("First call: {}", increment());
    println!("Second call: {}", increment());
    println!("Third call: {}", increment());

    //

    let numbers = [1, 2, 3, 4, 5];

    let squared: Vec<i32> = numbers.iter().map(|&x| x * x).collect();

    println!("Squared numbers: {:?}", squared); // Outputs: [1, 4, 9, 16, 25]

    let even_numbers: Vec<i32> = numbers.iter().filter(|&&x| x % 2 == 0).cloned().collect();

    println!("Even numbers: {:?}", even_numbers); // Outputs: [2, 4]
}
