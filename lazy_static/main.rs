
struct Bird;

impl Bird {
    pub fn new() -> Self {
        println!("new()");
        Self
    }
    
    pub fn fly(&self) {
        println!("fly()");
    }
}

impl Drop for Bird {
    fn drop(&mut self) {
        // lazy_static never calls drop!
        println!("drop()");
    }
}

lazy_static::lazy_static! {
    // BIRD is not destructed at all, memory or other resource leaks
    static ref BIRD: Bird = Bird::new();
}


fn main() {
    BIRD.fly();
}
