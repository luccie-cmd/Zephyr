pub mod driver;
pub mod syntax;
pub mod sema;
use std::ptr;
use std::str;
use std::str::Utf8Error;
use std::slice;
use std::ffi::CStr;
use std::fs;
use std::thread;
use driver::context::Context;
use driver::diag::DiagPrinter;
use std::error::Error;

#[repr(C)]
struct Args{
    verbose: bool,
    use_colors: bool,
    out_file: *const u8,
    file_paths_count: u8,
    file_paths: *const *const u8,
}

unsafe extern "C" {
    fn getArgs(argc: i32, argv: *const *const u8) -> *const Args;
}

fn convert(result: Result<&str, Utf8Error>) -> String {
    match result {
        Ok(s) => s.to_owned(), // Convert &str to String
        Err(_) => String::from("Invalid UTF-8"), // Handle errors gracefully
    }
}


fn main() {
    let result = thread::Builder::new()
        .stack_size(8 * 1024 * 1024) // Increase stack size
        .name("real main".to_string())
        .spawn(|| {
            real_main().map_err(|e| {
                eprintln!("Error: {}", e);
                e
            }) // Convert errors to an appropriate format
        })
        .unwrap()
        .join();

    match result {
        Ok(Ok(_)) => println!("Execution successful"),
        Ok(Err(_)) => println!("Caught stack overflow before crash!"),
        Err(_) => println!("Thread crashed due to stack overflow"),
    }
}

fn real_main() -> Result<(), Box<dyn Error + Send>> {
    let os_args: Vec<std::ffi::CString> = std::env::args()
        .map(|arg| std::ffi::CString::new(arg).unwrap())
        .collect();
    let c_args: Vec<*const u8> = os_args.iter()
        .map(|arg| arg.as_ptr() as *const u8)
        .collect();

    let args: Args;
    let verbose: bool;
    let use_colors: bool;
    let mut file_paths: Vec<String> = vec![];
    let _out_file: String;
    unsafe {
        args = ptr::read_unaligned(getArgs(c_args.len() as i32, c_args.as_ptr()));
        verbose = args.verbose;
        use_colors = args.use_colors;
        let file_paths_slice: &[*const u8] = std::slice::from_raw_parts(args.file_paths, args.file_paths_count as usize);
        for path in file_paths_slice {
            file_paths.push(convert(str::from_utf8(slice::from_raw_parts(*path, CStr::from_ptr(*path as *const i8).to_bytes().len()))));
        }
        _out_file = convert(str::from_utf8(slice::from_raw_parts(args.out_file, CStr::from_ptr(args.out_file as *const i8).to_bytes().len())));
    }
    for file_path in file_paths {
        let diagnostic_printer: DiagPrinter = DiagPrinter::new(verbose, use_colors, file_path.clone());
        let file_contents: String = fs::read_to_string(file_path.clone()).unwrap();
        let ctx: Context = Context::new(diagnostic_printer, file_contents);
        ctx.print_info(true);
        ctx.run();
    }
    Ok(())
}
