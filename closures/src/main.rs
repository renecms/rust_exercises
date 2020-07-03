use std::collections::HashMap;
use std::hash::Hash;
use std::thread;
use std::time::Duration;

struct Cacher<T, U, V>
where
    T: Fn(U) -> V,
{
    calculation: T,
    map: HashMap<U, V>,
}

impl<T, U: Eq + Hash + Copy, V: Copy> Cacher<T, U, V>
where
    T: Fn(U) -> V,
{
    fn new(calculation: T) -> Cacher<T, U, V> {
        Cacher {
            calculation,
            map: HashMap::new(),
        }
    }

    fn value(&mut self, arg: U) -> V {
        if self.map.contains_key(&arg) {
            self.map.get(&arg).unwrap().clone()
        } else {
            let v = (self.calculation)(arg);
            self.map.insert(arg, v);
            v
        }
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Next, do {} situps!", expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }
}

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);
}

#[test]
fn call_with_different_values_int() {
    let mut c = Cacher::new(|a| a);

    let v1 = c.value(1);
    let v2 = c.value(2);
    let v2_2 = c.value(2);
    let v3 = c.value(3);

    assert_eq!(v1, 1);
    assert_eq!(v2, v2_2);
    assert_eq!(v2, 2);
    assert_eq!(v3, 3);
}
#[test]
fn call_with_different_values_string() {
    let mut c = Cacher::new(|a| a);

    let v1 = c.value("a");
    let v2 = c.value("b");
    let v3 = c.value("c");

    assert_eq!(v1, "a");
    assert_eq!(v2, "b");
    assert_eq!(v3, "c");
}
