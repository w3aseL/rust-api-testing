use colored::*;
use log::Level;

fn get_level(level: Level) -> ColoredString {
    match level {
        Level::Info => level.as_str().yellow(),
        _ => level.as_str().red()
    }
}

pub fn setup_logger() -> Result<(), fern::InitError> {
    fern::Dispatch::new()
        .chain(
            fern::Dispatch::new()
            .format(|out, message, record| {
                out.finish(format_args!(
                    "{} [{}] {}",
                    chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string().blue(),
                    get_level(record.level()),
                    message
                ))
            })
            .level(log::LevelFilter::Warn)
            .level_for("api_testing", log::LevelFilter::Info)
            .chain(std::io::stdout())
        )
        .chain(
            fern::Dispatch::new()
            .format(|out, message, record| {
                out.finish(format_args!(
                    "{} [{}] {}",
                    chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                    record.level(),
                    message
                ))
            })
            .level(log::LevelFilter::Warn)
            .level_for("api_testing", log::LevelFilter::Info)
            .chain(fern::log_file("info.log")?)
        )
        .apply()?;
    Ok(())
}