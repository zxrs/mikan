macro_rules! print {
    ($($args:tt)*) => {{
        if let Some(writer) = WRITER.inner.borrow_mut().as_mut() {
            _ = write!(writer, "{}", format_args!($($args)*));
        }
    }}
}

macro_rules! println {
    ($($args:tt)*) => {
        print!("{}\n", format_args!($($args)*));
    };
}
