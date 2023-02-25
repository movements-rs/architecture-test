use common::{TestInput, TestOutput};

#[no_mangle]
pub extern "C" fn hello_world(input: TestInput) -> TestOutput {
    log::info!("I'm in test-parser::hello_world");

    TestOutput {
        baz: input.bar.into(),
    }
}
