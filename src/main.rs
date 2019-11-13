//// #![windows_subsystem = "windows"]

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

    app.add_menu_item("Quit", |window| {
        window.quit();
        Ok::<_, systray::Error>(())
    })?;

    println!("Ready");
    app.wait_for_message()?;
    Ok(())
}

 #[cfg(not(target_os = "windows"))]
 fn main() {
     panic!("Not implemented on this platform!");
 }