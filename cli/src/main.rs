mod cli_app;

use anyhow::Result;
use cli_app::CliApp;

fn main() -> Result<()> {
    let app = CliApp::build_cli()?;
    app.execute()?;
    Ok(())
}
