use actix_web::rt::System;
use dapla_server::settings::Settings;
use flexi_logger::{Duplicate, FileSpec, Logger};
use log::info;

mod panic;

#[cfg_attr(target_os = "android", ndk_glue::main(backtrace = "on"))]
pub fn main() {
    Logger::try_with_env_or_str("info,regalloc=warn,wasmer_compiler_cranelift=warn")
        .unwrap()
        .log_to_file(
            FileSpec::default()
                .directory("/sdcard/Android/data/rust.dapla_mobile/files")
                .basename("dapla")
                .suppress_timestamp()
                .suffix("log"),
        )
        .duplicate_to_stdout(Duplicate::All)
        .start()
        .unwrap();

    panic::set_logger_hook();

    info!("Load settings file");
    let settings = Settings::new("/sdcard/Android/data/rust.dapla_mobile/files/settings.toml")
        .expect("Settings should be configured");

    info!("Create actix system");
    System::new()
        .block_on(async move { dapla_server::run(settings).await })
        .expect("Dapla run error")
}
