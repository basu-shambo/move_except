use super::*;


#[test]
fn test_logger_basic() {
    let mut buf = Vec::new();
    Logger::with(Level::Warn, &mut buf).error("This is an error").warn("This is a warning").info("This is an info").debug("This is for debugging").log();
    let output = String::from_utf8(buf).unwrap();
    assert_eq!(output, "This is an errorThis is a warning\n");
}

#[test]
fn test_logger_levels() {
    let mut buf = Vec::new();
    Logger::with(Level::Error, &mut buf).error("This is an error").warn("This is a warning").info("This is an info").debug("This is for debugging").log();
    let output = str::from_utf8(&buf).unwrap();
    assert_eq!(output, "This is an error\n");

    buf.clear();
    Logger::with(Level::Warn, &mut buf).error("This is an error").warn("This is a warning").info("This is an info").debug("This is for debugging").log();
    let output = str::from_utf8(&buf).unwrap();
    assert_eq!(output, "This is an errorThis is a warning\n");

    buf.clear();
    Logger::with(Level::Info, &mut buf).error("This is an error").warn("This is a warning").info("This is an info").debug("This is for debugging").log();
    let output = str::from_utf8(&buf).unwrap();
    assert_eq!(output, "This is an errorThis is a warningThis is an info\n");

    buf.clear();
    Logger::with(Level::Debug, &mut buf).error("This is an error").warn("This is a warning").info("This is an info").debug("This is for debugging").log();
    let output = str::from_utf8(&buf).unwrap();
    assert_eq!(output, "This is an errorThis is a warningThis is an infoThis is for debugging\n");
}

#[test]
fn single_levels() {
    let mut buf = Vec::new();
    Logger::with(Level::Error, &mut buf).error("This is an error").log();
    let err_output = str::from_utf8(&buf).unwrap().to_string();

    buf.clear();

    Logger::with(Level::Warn, &mut buf).warn("This is a warning").log();
    let warn_output = str::from_utf8(&buf).unwrap().to_string();

    buf.clear();

    Logger::with(Level::Info, &mut buf).info("This is an info").log();
    let info_output = str::from_utf8(&buf).unwrap().to_string();

    buf.clear();

    Logger::with(Level::Debug, &mut buf).debug("This is for debugging").log();
    let debug_output = str::from_utf8(&buf).unwrap().to_string();

    buf.clear();

    assert_eq!(err_output, "This is an error\n");
    assert_eq!(warn_output, "This is a warning\n");
    assert_eq!(info_output, "This is an info\n");
    assert_eq!(debug_output, "This is for debugging\n");
}

//Add test to test only
    // At the begining and in the middle of the different ones
//Add test to test mismatched levels with and without only

