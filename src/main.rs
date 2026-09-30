/// Knowledge is power
fn main() -> anyhow::Result<()> {
    cli_log::init_cli_log!();
    if let Err(e) = bacon::run() {
        if e.downcast_ref::<std::io::Error>()
            .is_some_and(|e| e.kind() == std::io::ErrorKind::BrokenPipe)
        {
            std::process::exit(141);
        }
        return Err(e);
    }
    cli_log::info!("bye");
    Ok(())
}
