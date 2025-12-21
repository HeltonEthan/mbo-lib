use dbn_mbo_engine::prelude::*;
use chrono::NaiveDate;
use std::path::PathBuf;
use dbn::record::MboMsg;

// cargo test -p dbn_mbo_engine --test integration_engine engine_test --release
#[test]
fn engine_test() -> anyhow::Result<()> {
    let cfg = Config::new(
        PathBuf::from("C:/Users/helto/GLBX-20250915-NGKNUL4VBG"),
        NaiveDate::from_ymd_opt(2025, 05, 12).unwrap(),
        NaiveDate::from_ymd_opt(2025, 05, 17).unwrap(),
        0,
    );
    run(logic, &cfg)?;
    Ok(())
}

fn logic(mbo: &MboMsg) -> Signal {
    _ = mbo;
    Signal::None
}
