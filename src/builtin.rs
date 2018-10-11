use std::convert::AsRef;
use std::process;

pub fn exit<T: AsRef<str>>(args: Vec<T>) -> ! {
    let exitcode = if args.len() > 2 {
        args[1]
            .as_ref()
            .parse::<i32>()
            .expect(&format!("{} is not a numeric argument", args[1].as_ref()))
    } else {
        0
    };

    process::exit(exitcode)
}
