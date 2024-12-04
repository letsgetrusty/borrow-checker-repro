struct AnotherStruct;

struct MyStruct {
    field_1: AnotherStruct,
}

impl MyStruct {
    fn new() -> MyStruct {
        MyStruct {
            field_1: AnotherStruct,
        }
    }

    fn method_1(&self) -> &AnotherStruct {
        &self.field_1
    }

    fn method_2(&mut self, x: &AnotherStruct) {
        println!("My method 2");
    }

    fn method_3(&mut self) {
        let x = self.method_1();
        self.method_2(x);
    }
}

fn main() {
    println!("Hello, world!");
}
