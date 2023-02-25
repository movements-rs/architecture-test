use common::{TestInput, TestOutput};

fn main() {
    env_logger::init();

    log::info!("I'm in movements");

    let lib = unsafe {
        libloading::Library::new(
            "C:/Users/jamwa/Repositories/movements-test/target/debug/deps/test_parser.dll",
        )
    }
    .unwrap();

    let func: libloading::Symbol<unsafe extern "C" fn(TestInput) -> TestOutput> =
        unsafe { lib.get(b"hello_world\0") }.unwrap();

    let input = TestInput {
        foo: 12341234,
        bar: 56785678,
    };

    let output = unsafe { func(input) };

    log::info!("Output: {:?}", output);
}
