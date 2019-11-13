//// #![windows_subsystem = "windows"] //we don't need it until we are keeping console window here

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
        // TODO: Execute connect
        app.set_icon_from_file("on.ico")?;
        println!("Done");
        Ok::<_, systray::Error>(())
    })?;

    app.add_menu_item("Disconnect", |app| {
        println!("Disconnecting...");
        // TODO: Execute disconnect
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
        let msg = match  visibility {
            false => winapi::um::winuser::SW_HIDE,
            _ => winapi::um::winuser::SW_SHOWNORMAL,
        };
        unsafe {
            user32::ShowWindow(window, msg);
        }
    }
}

#[cfg(not(target_os = "windows"))]
fn main() {
    panic!("Not implemented on this platform!");
}