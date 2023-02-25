#[derive(Debug)]
#[repr(C)]
pub struct TestInput {
    pub foo: i32,
    pub bar: u32,
}

#[derive(Debug)]
#[repr(C)]
pub struct TestOutput {
    pub baz: i64,
}
