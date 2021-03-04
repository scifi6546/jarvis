use log::{info, Level};

pub fn new() {
    console_log::init_with_level(Level::Debug).expect("failed to init log");
    info!("hello world");
}
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
