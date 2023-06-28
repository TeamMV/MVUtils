/// This macro creates a libC entry point into the program. To use this macro, you must indicate to the compiler that
/// there is no main function by using `#![no_main]`. This macro will create a C function called `main` and invoke the inner
/// function you provide it. The inner function can take no arguments, or an argument which can be one of a few types:
/// - `impl Iterator<Item = String>` - This will provide you with an iterator with heap allocated strings in it.
/// - `impl Iterator<Item = &'static str>` - This will provide you with an iterator with the raw C strings in it as String slices.
/// - `Vec<String>` - This will provide you with a vector of heap allocated strings.
/// - `ArgParser` - This will provide an argument parser which will allow you to instantly parse the arguments.
/// The function may also return an `i32` type, which will pass the return value to the C function.
///
/// ```
/// #![no_main]
///
/// use mvutils::cinit;
///
/// cinit!(
///     pub fn main(args: impl Iterator<Item = &'static str>) -> i32 {
///         println!("Hello, world!");
///         if args.any(|arg| arg == "success") {
///             0
///         } else {
///             1
///         }
///     }
/// );
/// ```
///
/// In this example, the program asks for an iterator over raw sting slices, which means no allocation is necessary, and it
/// returns an `i32`, which is propagated and returned from the C entry point.
///
/// ```
/// #![no_main]
///
/// use mvutils::cinit;
/// use mvutils::args::ArgParser;
///
/// cinit!(
///     pub fn main(args: ArgParser) {
///         // do stuff with the parser
///     }
/// );
/// ```
///
/// In this example, the program asks for an `ArgParser`, which is a struct from this crate. Unlike the previous example,
/// this function doesn't return anything, so the C entry point will return 0.
#[macro_export]
macro_rules! cinit {
    (
        $vis:vis fn $name:ident() $body:block
    ) => {
        mod init {
            use super::*;
            pub fn $name() $body
        }

        #[no_mangle]
        pub unsafe extern "C" fn main(argc: i32, argv: *const *const std::ffi::c_char) -> i32 {
            init::$name();
            0
        }
    };
    (
        $vis:vis fn $name:ident() -> $ret:ty $body:block
    ) => {
        mod init {
            use super::*;
            pub fn $name() -> $ret $body
        }

        #[no_mangle]
        pub unsafe extern "C" fn main(argc: i32, argv: *const *const std::ffi::c_char) -> i32 {
            init::$name()
        }
    };
    (
        $vis:vis fn $name:ident($arg:ident: $($t:tt)+) $body:block
    ) => {
        mod init {
            use super::*;
            pub fn $name($arg: $($t)+) $body
        }

        #[no_mangle]
        pub unsafe extern "C" fn main(argc: i32, argv: *const *const std::ffi::c_char) -> i32 {
            init::$name(cinit!(argv, argc, $($t)+));
            0
        }
    };
    (
        $vis:vis fn $name:ident($arg:ident: $t:tt) -> $ret:ty $body:block
    ) => {
        mod init {
            use super::*;
            pub fn $name($arg: $($t)+) -> $ret $body
        }

        #[no_mangle]
        pub unsafe extern "C" fn main(argc: i32, argv: *const *const std::ffi::c_char) -> i32 {
            init::$name(cinit!(argv, argc, $($t)+))
        }
    };
    ($v:ident, $c:ident, impl Iterator<Item = &'static str>) => {
        {
            let arr = core::slice::from_raw_parts($v, $c as usize);
            arr.iter().map(|arg| {
                std::str::from_utf8_unchecked(std::ffi::CStr::from_ptr(*arg).to_bytes())
            })
        }
    };
    ($v:ident, $c:ident, impl Iterator<Item = String>) => {
        {
            let arr = core::slice::from_raw_parts($v, $c as usize);
            arr.iter().map(|arg| {
                std::str::from_utf8_unchecked(std::ffi::CStr::from_ptr(*arg).to_bytes()).to_string()
            })
        }
    };
    ($v:ident, $c:ident, Vec<String>) => {
        {
            let arr = core::slice::from_raw_parts($v, $c as usize);
            arr.iter().map(|arg| {
                std::str::from_utf8_unchecked(std::ffi::CStr::from_ptr(*arg).to_bytes()).to_string()
            }).collect::<Vec<_>>()
        }
    };
    ($v:ident, $c:ident, ArgParser) => {
        {
            let arr = core::slice::from_raw_parts($v, $c as usize);
            $crate::args::ParseArgs::parse_args(arr.iter().map(|arg| {
                std::str::from_utf8_unchecked(std::ffi::CStr::from_ptr(*arg).to_bytes())
            }))
        }
    };
}