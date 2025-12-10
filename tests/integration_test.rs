use mbo_engine;
use color_eyre::eyre::Result;

#[test]
fn integration_test() -> Result<()> {
    let config0 = mbo_engine::Config::new(
        "C:/Users/helto/GLBX-20250915-NGKNUL4VBG".to_string(),
        "2025-05-12".to_string(),
        "2025-05-17".to_string(),
    )?;

    println!("config: {:#?}", config0);

    mbo_engine::run(config0)?;
    
    Ok(())
}
