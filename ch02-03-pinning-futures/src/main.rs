use std::ptr;

struct SelfReferential {
    data: String,
    self_pointer: *const String,
}

impl SelfReferential {
    fn new(data: String) -> SelfReferential {
        let mut sr = SelfReferential {
            data,
            self_pointer: ptr::null(),
        };
        sr.self_pointer = &sr.data as *const String;
        sr
    }

    fn print(&self) {
        unsafe {
            // println!("{:p}", self.self_pointer);
            println!("{}", *self.self_pointer);
        }
    }
}

fn move_some_memory(sr: SelfReferential) {
    sr.print();
}

fn main() {
    let first = SelfReferential::new("first".to_string());
    first.print();
    move_some_memory(first);
}
