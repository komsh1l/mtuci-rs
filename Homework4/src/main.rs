struct Vector<T> {
    nk: Vec<T>
}

impl<T: Clone> Vector<T> {
    fn new() -> Vector<T>{
        Vector {
            nk: Vec::new()
        }
    }

    fn with_capacity(capacity: usize) -> Vector<T>{
        Vector {
            nk: Vec::with_capacity(capacity)
        }
    }

    fn push(&mut self, value: T) {
        self.nk.push(value)
    }

    fn pop(&mut self) -> Option<T> {
        self.nk.pop()
    }

     fn remove(&mut self, index: usize) -> Option<T> {
        if index >= self.nk.len() {
            None
        } else {
            Some(self.nk.remove(index))
        }
     }

     fn get(&self, index: usize) -> Option<&T> {
        self.nk.get(index)
     }

     fn resize(&mut self, new_size: usize, value: T) {
        self.nk.resize(new_size, value)
     }

}
fn main() {
    let mut vec = Vector::new();
    let mut vector = Vector::with_capacity(5);

    vec.push(1);
    vec.push(2);
    vec.push(3);

    println!("{:?}", vec.pop());
    println!("{:?}", vec.remove(1));
    println!("{:?}", vec.get(1));

    vector.resize(3, 5);
    println!("{:?}", vector.pop());
}