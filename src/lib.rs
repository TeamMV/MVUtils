mod utils;
mod version;
mod logger;
mod args;

#[cfg(test)]
mod tests {
    use once_cell::sync::Lazy;
    use crate::version::Version;
    use crate::*;
    use crate::logger::*;
    use crate::logger::LogLevel::DEBUG;
    use crate::utils::*;

    #[test]
    fn it_works() {
        let logger = Logger::new(|l, s| {
            println!("{}", s);
        });
        logger.debug("Hello!".to_string());
        logger.info("Hello!".to_string());
        logger.warn("Hello!".to_string());
        logger.error("Hello!".to_string());
    }

    #[test]
    fn it_works2() {

    }
}
