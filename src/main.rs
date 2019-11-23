//// #![windows_subsystem = "windows"] //we don't need it until we are keeping console window here

#[cfg(not(target_os = "windows"))]
fn main() {
    panic!("Not implemented on this platform!");
}

#[cfg(target_os = "windows")]
fn main() -> Result<(), systray::Error> {
    let mut app;
    match systray::Application::new() {
        Ok(w) => app = w,
        Err(_) => panic!("Can't create window!"),
    }
    app.set_icon_from_file("off.ico")?;

    app.add_menu_item("Connect", |app| {
        println!("Connecting...");

        if execute_before_connect() {
            if execute_connect() {
                if execute_after_connect() {
                    heart_beat(true);
                }
            }
        }

        // TODO: Execute connect
        let output = std::process::Command::new("ping").arg("localhost").spawn();
        match output {
            Err(e) => println!("Error: {:?}", e),
            Ok(mut c) => {
                println!("Ok: {:?}", c);
                let exit_status = c.wait().unwrap();
                println!("ExitStatus: {:?}", exit_status);
            }
        }

        app.set_icon_from_file("on.ico")?;
        println!("Done");
        Ok::<_, systray::Error>(())
    })?;

    app.add_menu_item("Disconnect", |app| {
        println!("Disconnecting...");
// TODO: Execute disconnect

        if execute_before_disconnect() {
            if execute_disconnect() {
                if execute_after_disconnect() {
                    heart_beat(false);
                }
            }
        }

        app.set_icon_from_file("off.ico")?;
        println!("Done");
        Ok::<_, systray::Error>(())
    })?;

    app.add_menu_separator()?;

    app.add_menu_item("Hide console", |_| {
        set_console_visibility(false);
        Ok::<_, systray::Error>(())
    })?;

    app.add_menu_item("Show console", |_| {
        set_console_visibility(true);
        Ok::<_, systray::Error>(())
    })?;

    app.add_menu_separator()?;

    app.add_menu_item("Quit", |window| {
        window.quit();
        Ok::<_, systray::Error>(())
    })?;

    println!("Ready");
    app.wait_for_message()?;
    Ok(())
}

fn set_console_visibility(visibility: bool) {
    let window = unsafe { kernel32::GetConsoleWindow() };
// https://msdn.microsoft.com/en-us/library/windows/desktop/ms633548%28v=vs.85%29.aspx
    if window != std::ptr::null_mut() {
        let msg = match visibility {
            false => winapi::um::winuser::SW_HIDE,
            _ => winapi::um::winuser::SW_SHOWNORMAL,
        };
        unsafe { user32::ShowWindow(window, msg); }
    }
}

enum Phase {
    Before,
    On,
    After
}

enum Operation {
    Connect(Phase),
    Disconnect(Phase)
}

struct Status {
    Current: Option<Operation>,
    Connected: bool,
}

