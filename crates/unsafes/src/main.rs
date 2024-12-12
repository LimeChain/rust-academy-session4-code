// Демонстриране на различни функции на unsafe Rust

#![allow(unused)]

// extern "C" {
//     fn neshto();
// }

// 1. Raw pointers и основна unsafe употреба
fn raw_pointers_example() {
    // Създаване на raw pointer-и (и променливи, и непроменливи)
    let mut num = 5;

    // unsafe {
    //     neshto();
    // }

    // Raw pointer-ите могат да бъдат създадени в safe код, но дереференцирането изисква unsafe
    let raw_const_ptr: *const i32 = &num;
    let raw_mut_ptr: *mut i32 = &mut num;

    // Дереференцирането изисква unsafe блок
    unsafe {
        println!("Константен сур показалец: {}", *raw_const_ptr);
        *raw_mut_ptr += 1;
        println!("Променена стойност: {}", *raw_mut_ptr);
    }
}

// 2. Извикване на unsafe функции
unsafe fn dangerous_function() {
    println!("Това е unsafe функция, която може да върши рискови неща!");
}

// 3. Безопасна абстракция върху unsafe код (имитация на Vec)
struct RawVec<T> {
    ptr: *mut T,
    capacity: usize,
}

impl<T> RawVec<T> {
    fn new() -> Self {
        // Еквивалент на Raw алокация във Vec
        let ptr = unsafe { std::alloc::alloc(std::alloc::Layout::new::<T>()) } as *mut T;
        RawVec { ptr, capacity: 1 }
    }

    // Безопасен wrapper около потенциално unsafe операции
    fn get(&self, index: usize) -> Option<&T> {
        unsafe {
            // Проверки за граници и нулевост
            if self.ptr.is_null() || index >= self.capacity {
                None
            } else {
                Some(&*self.ptr.add(index))
            }
        }
    }
}

// 4. Външни функционални извиквания (FFI)
extern "C" {
    // Деклариране на външна C функция
    fn abs(input: i32) -> i32;
}

// 5. Променливи статични променливи
static mut GLOBAL_COUNTER: i32 = 0;

fn increment_global_counter() {
    unsafe {
        GLOBAL_COUNTER += 1;
    }
}

// 6. Unsafe trait имплементация
// Представете си trait, който изисква ръчно управление на паметта
/// # Safety
/// нещо умно
unsafe trait UnsafeMarker {
    unsafe fn dangerous_method(&mut self);
}

struct UnsafeStruct;

// Трябва да използваме 'unsafe' при имплементиране на unsafe trait
unsafe impl UnsafeMarker for UnsafeStruct {
    unsafe fn dangerous_method(&mut self) {
        println!("Извършване на нещо, дет може да гръмне");
    }
}

// 7. Пример, вдъхновен от реалния свят: Опростен Mutex (подобен на std::sync::Mutex)
use std::cell::UnsafeCell;
use std::sync::atomic::{AtomicBool, Ordering};

struct RawMutex<T> {
    locked: AtomicBool,
    data: UnsafeCell<T>,
}

impl<T> RawMutex<T> {
    fn new(data: T) -> Self {
        RawMutex {
            locked: AtomicBool::new(false),
            // Като RefCell от миналият път, ама директно ни дава raw pointer
            data: UnsafeCell::new(data),
        }
    }

    #[allow(clippy::mut_from_ref)]
    fn lock(&self) -> &mut T {
        while self
            .locked
            .compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
            .is_err()
        {
            // Изчакване докато lock бъде придобит
        }

        unsafe { &mut *self.data.get() }
    }

    fn unlock(&self) {
        self.locked.store(false, Ordering::Release);
    }
}

fn main() {
    // Демонстриране на различни unsafe Rust функции
    raw_pointers_example();

    // Извикване на unsafe функция
    unsafe {
        dangerous_function();
    }

    // Външно функционално извикване
    unsafe {
        println!("Абсолютна стойност: {}", abs(-42));
    }

    // Променлива статична променлива
    unsafe {
        increment_global_counter();
        println!("Глобален брояч: {}", GLOBAL_COUNTER);
    }

    // Unsafe trait
    let mut unsafe_instance = UnsafeStruct;
    unsafe {
        unsafe_instance.dangerous_method();
    }
}
