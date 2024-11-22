use std::thread;

fn sum<T: std::ops::AddAssign + Copy + Default>(arr: &[T]) -> T {
    let mut result: T = T::default();
    for item in arr {
        result += *item
    }
    result
}

fn main() {
    let first = [1, 2, 3];
    let sec = [3, 4, 5_u64];
    println!(
        "number of threads: {} number of physical cores: {}",
        num_cpus::get(),
        num_cpus::get_physical()
    );

    let mut handles = vec![];

    for i in 0..num_cpus::get() {
        let handle = thread::spawn(move || {
          println!("hello from thread number {i}");
          println!("sum: {}", sum(&sec));
          println!("sum: {}", sum(&first));
        });
        handles.push(handle);
    }
    for handle in handles {
      handle.join().unwrap();
    }
}
