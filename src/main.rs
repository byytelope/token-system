pub use crate::queue::Queue;

mod queue;

fn main() {
    let mut q = Queue::new();
    println!("{:?}", q);
    let a = q.add_room(10, "Dr. Ahmed".to_string());
    println!("{:?}", a);
    let b = q.increment(10);
    println!("{:?}", b);
}
