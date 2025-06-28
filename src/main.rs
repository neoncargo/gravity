mod app;
mod nannou_adapter;

fn main() {
    setup_logger().unwrap_or_else(|e| println!("Logger failed: {e}"));

    nannou::app(nannou_adapter::model)
    .event(nannou_adapter::event)
    .update(nannou_adapter::update)
    .run();
}

fn setup_logger() -> Result<(), fern::InitError> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}:{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.file().unwrap_or("unknown"),
                record.line().unwrap_or(0),
                fern::colors::ColoredLevelConfig::new().color(record.level()),
                message
            ))
        })
        .level_for("gravity", log::LevelFilter::Debug)
        .level(log::LevelFilter::Off)
        .chain(std::io::stdout())
        .chain(fern::log_file("output.log")?)
        .apply()?;
    Ok(())
}
