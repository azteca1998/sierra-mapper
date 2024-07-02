struct MyStruct {
    x: u32,
    y: u32,
}

enum MyEnum {
    X: u32,
    Y: u32,
}

fn main(x: MyStruct, y: MyEnum) -> (MyStruct, MyEnum) {
    (x, y)
}

fn main2(x: Box<()>) -> Box<()> {
    x
}
