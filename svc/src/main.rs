#[macro_use] extern crate windows_service;

use dotenv::dotenv;
use std::ffi::OsString;
use std::time::Duration;
use windows_service::service_dispatcher;
use windows_service::service::{
    /*ServiceControl,*/ ServiceControlAccept, ServiceExitCode, ServiceState, ServiceStatus,
    ServiceType,
};
use windows_service::service_control_handler::{self, ServiceControlHandlerResult};

static SERVICE_NAME: &str = "dial-auto-svc";
define_windows_service!(ffi_service_main, my_service_main);

fn my_service_main(arguments: Vec<OsString>) {
    if let Err(e) = run_service(arguments) {
        log::error!("{:?}", e);
    }
}

fn run_service(_arguments: Vec<OsString>) -> windows_service::Result<()> {
    let event_handler = move |control_event| -> ServiceControlHandlerResult {
        match control_event {
//            ServiceControl::Stop | ServiceControl::Interrogate => {
//                ServiceControlHandlerResult::NoError
//            }

            evt => {
                log::info!("{:?}", evt);
                ServiceControlHandlerResult::NoError
            }
        }
    };

    // Register system service event handler
    let status_handle = service_control_handler::register(SERVICE_NAME, event_handler)?;

    let next_status = ServiceStatus {
        service_type: ServiceType::OWN_PROCESS, // Should match the one from system service registry
        current_state: ServiceState::Running, // The new state
        controls_accepted: ServiceControlAccept::STOP, // Accept stop events when running
        exit_code: ServiceExitCode::Win32(0), // Used to report an error when starting or stopping only, otherwise must be zero
        checkpoint: 0, // Only used for pending states, otherwise must be zero
        wait_hint: Duration::default(), // Only used for pending states, otherwise must be zero
    };

    // Tell the system that the service is running now
    status_handle.set_service_status(next_status)?;

    // Do some work

    Ok(())
}

#[cfg(windows)]
fn main() -> Result<(), windows_service::Error> {
    dotenv().ok();
    init_fern().ok();
    log::info!("started");
    // Register generated `ffi_service_main` with the system and start the service, blocking
    // this thread until the service is stopped.
    service_dispatcher::start(SERVICE_NAME, ffi_service_main)?;
    Ok(())
}

#[cfg(not(windows))]
fn main() {
    panic!("This program is only intended to run on Windows.");
}

fn init_fern() -> Result<(), fern::InitError> {
    let tmp_dir = std::env::var("temp").unwrap();
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                record.level(),
                message
            ))
        })
        .level(log::LevelFilter::Debug)
        .chain(fern::log_file(tmp_dir + "\\" + SERVICE_NAME + ".log")?)
        .apply()?;
    Ok(())
}
