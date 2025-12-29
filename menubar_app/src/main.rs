use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use sysinfo::{System, ProcessesToUpdate, Signal};
use tray_icon::{TrayIconBuilder, menu::{Menu, MenuEvent, MenuItem, CheckMenuItem, PredefinedMenuItem, MenuId}};
use tray_icon::Icon;
use auto_launch::AutoLaunch;
use serde::{Deserialize, Serialize};
use std::fs;

const APP_NAME: &str = "Lapsus Control";
const APP_VERSION: &str = env!("CARGO_PKG_VERSION");
const CONFIG_FILE: &str = ".lapsus_menubar_config.json";

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Config {
    start_at_login: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    lapsus_rust_path: Option<String>,
    #[serde(default = "default_show_dock_icon")]
    show_dock_icon: bool,
}

fn default_show_dock_icon() -> bool {
    false // Default: hide from dock
}

impl Default for Config {
    fn default() -> Self {
        Self {
            start_at_login: false,
            lapsus_rust_path: None,
            show_dock_icon: false,
        }
    }
}

#[derive(Clone)]
struct AppState {
    lapsus_path: PathBuf,
    config_path: PathBuf,
    config: Arc<Mutex<Config>>,
    auto_launcher: Arc<Mutex<AutoLaunch>>,
    icon_enabled: Icon,
    icon_disabled: Icon,
}

impl AppState {
    fn find_lapsus_rust(app_dir: &std::path::Path) -> Result<PathBuf, Box<dyn std::error::Error>> {
        // Check config file first for custom path
        if let Some(home) = dirs::home_dir() {
            let config_path = home.join(CONFIG_FILE);
            if config_path.exists() {
                if let Ok(content) = fs::read_to_string(&config_path) {
                    if let Ok(config) = serde_json::from_str::<Config>(&content) {
                        if let Some(custom_path) = config.lapsus_rust_path {
                            let path = PathBuf::from(custom_path);
                            if path.exists() {
                                return Ok(path);
                            }
                        }
                    }
                }
            }
        }

        // List of possible locations to check (in order of priority)
        let mut possible_paths: Vec<PathBuf> = Vec::new();
        
        // 1. Bundled with the app (same directory as executable)
        possible_paths.push(app_dir.join("lapsus_rust"));
        
        // 2. In Resources folder of app bundle
        if let Some(parent) = app_dir.parent() {
            possible_paths.push(parent.join("Resources/lapsus_rust"));
        }
        
        // 3. Original development location (parent of menubar_app)
        if let Some(parent) = app_dir.parent() {
            if let Some(grandparent) = parent.parent() {
                possible_paths.push(grandparent.join("lapsus_rust"));
            }
        }
        
        // 4. Sibling to app bundle (if in /Applications)
        if let Some(parent) = app_dir.parent() {
            if let Some(gp) = parent.parent() {
                if let Some(ggp) = gp.parent() {
                    possible_paths.push(ggp.join("lapsus_rust"));
                }
            }
        }
        
        // 5. Fixed path to development location
        possible_paths.push(PathBuf::from("/Users/ryder/bin/lapsus/lapsus_rust"));
        
        // 6. In user's bin
        if let Some(home) = dirs::home_dir() {
            possible_paths.push(home.join("bin/lapsus/lapsus_rust"));
        }
        
        // 7. In /usr/local/bin
        possible_paths.push(PathBuf::from("/usr/local/bin/lapsus_rust"));

        // Check each location
        for path in possible_paths {
            if path.exists() {
                return Ok(path);
            }
        }

        Err("lapsus_rust not found in any expected location. Please set the path in config or bundle it with the app.".into())
    }

    fn new() -> Result<Self, Box<dyn std::error::Error>> {
        // Get paths
        let current_exe = std::env::current_exe()?;
        let app_dir = current_exe.parent().ok_or("Cannot get parent directory")?;
        
        // Try multiple locations for lapsus_rust
        let lapsus_path = Self::find_lapsus_rust(app_dir)?;

        let config_path = dirs::home_dir()
            .ok_or("Cannot find home directory")?
            .join(CONFIG_FILE);

        // Load or create config
        let config = if config_path.exists() {
            let content = fs::read_to_string(&config_path)?;
            serde_json::from_str(&content).unwrap_or_default()
        } else {
            Config::default()
        };

        // Setup auto-launch
        let auto_launcher = AutoLaunch::new(
            APP_NAME,
            &current_exe.to_string_lossy(),
            false, // use_launch_agent
            &[] as &[&str],
        );

        // Load icons
        let icons_dir = if current_exe.to_string_lossy().contains(".app/Contents/MacOS") {
            // Running from app bundle
            current_exe.parent()
                .and_then(|p| p.parent())
                .map(|p| p.join("Resources"))
                .unwrap_or_else(|| app_dir.join("../icons"))
        } else {
            // Running from cargo build
            app_dir.join("../icons")
        };
        
        let icon_enabled = load_icon(icons_dir.join("cursor_enabled.png"))?;
        let icon_disabled = load_icon(icons_dir.join("cursor_disabled.png"))?;

        Ok(Self {
            lapsus_path,
            config_path,
            config: Arc::new(Mutex::new(config)),
            auto_launcher: Arc::new(Mutex::new(auto_launcher)),
            icon_enabled,
            icon_disabled,
        })
    }

    fn save_config(&self) -> Result<(), Box<dyn std::error::Error>> {
        let config = self.config.lock().unwrap();
        let content = serde_json::to_string_pretty(&*config)?;
        fs::write(&self.config_path, content)?;
        Ok(())
    }

    fn is_lapsus_running(&self) -> bool {
        // First check if launchd service is loaded
        if let Ok(output) = Command::new("launchctl")
            .args(["list", "com.lapsus.rust"])
            .output()
        {
            if output.status.success() {
                // Service exists, check if it's running (has a PID)
                let stdout = String::from_utf8_lossy(&output.stdout);
                // Output format: "PID\tStatus\tLabel" or "-\tStatus\tLabel"
                // If first field is a number (PID), it's running
                if let Some(first_line) = stdout.lines().next() {
                    if let Some(pid_str) = first_line.split_whitespace().next() {
                        if pid_str.parse::<i32>().is_ok() {
                            return true;
                        }
                    }
                }
            }
        }
        
        // Fallback: check process list (for manual starts)
        let mut sys = System::new_all();
        sys.refresh_processes(ProcessesToUpdate::All, true);
        
        sys.processes().values().any(|process| {
            let name = process.name().to_string_lossy();
            name.contains("lapsus_rust") || name == "lapsus_rust"
        })
    }

    fn start_lapsus(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Check if LaunchAgent exists
        let plist_path = dirs::home_dir()
            .ok_or("Cannot find home directory")?
            .join("Library/LaunchAgents/com.lapsus.rust.plist");
        
        if plist_path.exists() {
            // Use launchctl to load the service
            let output = Command::new("launchctl")
                .args(["load", plist_path.to_str().unwrap()])
                .output()?;
            
            if !output.status.success() {
                let stderr = String::from_utf8_lossy(&output.stderr);
                // Ignore "already loaded" errors
                if !stderr.contains("Already loaded") && !stderr.contains("service already loaded") {
                    return Err(format!("Failed to load service: {}", stderr).into());
                }
            }
            
            Ok(())
        } else {
            // Fallback: start process directly if no LaunchAgent
            if !self.lapsus_path.exists() {
                return Err(format!("lapsus_rust not found at: {:?}", self.lapsus_path).into());
            }

            Command::new(&self.lapsus_path)
                .stdin(Stdio::null())
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .spawn()?;

            Ok(())
        }
    }

    fn stop_lapsus(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Check if LaunchAgent exists
        let plist_path = dirs::home_dir()
            .ok_or("Cannot find home directory")?
            .join("Library/LaunchAgents/com.lapsus.rust.plist");
        
        if plist_path.exists() {
            // Use launchctl to unload the service
            let output = Command::new("launchctl")
                .args(["unload", plist_path.to_str().unwrap()])
                .output()?;
            
            if !output.status.success() {
                let stderr = String::from_utf8_lossy(&output.stderr);
                // Ignore "not loaded" errors
                if !stderr.contains("Could not find") && !stderr.contains("not loaded") {
                    return Err(format!("Failed to unload service: {}", stderr).into());
                }
            }
            
            Ok(())
        } else {
            // Fallback: kill process directly if no LaunchAgent
            let mut sys = System::new_all();
            sys.refresh_processes(ProcessesToUpdate::All, true);
            
            let mut found = false;
            for (_pid, process) in sys.processes() {
                let name = process.name().to_string_lossy();
                if name.contains("lapsus_rust") || name == "lapsus_rust" {
                    if process.kill_with(Signal::Term).unwrap_or(false) {
                        found = true;
                    }
                }
            }

            if !found {
                return Err("lapsus_rust process not found".into());
            }

            Ok(())
        }
    }

    fn toggle_auto_launch(&self, enable: bool) -> Result<(), Box<dyn std::error::Error>> {
        let auto_launcher = self.auto_launcher.lock().unwrap();
        
        if enable {
            auto_launcher.enable()?;
        } else {
            auto_launcher.disable()?;
        }

        let mut config = self.config.lock().unwrap();
        config.start_at_login = enable;
        drop(config);
        
        self.save_config()?;
        Ok(())
    }

    fn get_current_icon(&self) -> Icon {
        if self.is_lapsus_running() {
            // When running, show FILLED icon
            self.icon_enabled.clone()
        } else {
            // When stopped, show OUTLINE icon
            self.icon_disabled.clone()
        }
    }
}

fn load_icon(path: PathBuf) -> Result<Icon, Box<dyn std::error::Error>> {
    let image = image::open(&path)?;
    let rgba = image.to_rgba8();
    let (width, height) = rgba.dimensions();
    
    Icon::from_rgba(rgba.into_raw(), width, height)
        .map_err(|e| format!("Failed to create icon: {:?}", e).into())
}

fn build_menu(state: &AppState) -> Result<Menu, Box<dyn std::error::Error>> {
    let menu = Menu::new();
    
    let is_running = state.is_lapsus_running();
    
    // Enable/Disable items
    if is_running {
        let disable_item = MenuItem::with_id(
            MenuId::new("disable"),
            "Disable Lapsus",
            true,
            None
        );
        menu.append(&disable_item)?;
    } else {
        let enable_item = MenuItem::with_id(
            MenuId::new("enable"),
            "Enable Lapsus",
            true,
            None
        );
        menu.append(&enable_item)?;
    }
    
    menu.append(&PredefinedMenuItem::separator())?;
    
    // Start at Login checkbox
    let config = state.config.lock().unwrap();
    let start_at_login = CheckMenuItem::with_id(
        MenuId::new("start_at_login"),
        "Start at Login",
        true,
        config.start_at_login,
        None
    );
    let show_dock_icon = CheckMenuItem::with_id(
        MenuId::new("show_dock_icon"),
        "Show Dock Icon",
        true,
        config.show_dock_icon,
        None
    );
    drop(config);
    menu.append(&start_at_login)?;
    menu.append(&show_dock_icon)?;
    
    menu.append(&PredefinedMenuItem::separator())?;
    
    // About
    let about_item = MenuItem::with_id(
        MenuId::new("about"),
        "About",
        true,
        None
    );
    menu.append(&about_item)?;
    
    // Quit
    let quit_item = MenuItem::with_id(
        MenuId::new("quit"),
        "Quit",
        true,
        None
    );
    menu.append(&quit_item)?;
    
    Ok(menu)
}

fn show_about_dialog() {
    #[cfg(target_os = "macos")]
    {
        use std::process::Command;
        let message = format!(
            "{} - Version {}\\n\\nMenu bar app to control lapsus_rust\\n\\nhttps://github.com/margooey/lapsus_rust",
            APP_NAME, APP_VERSION
        );
        
        let script = format!(
            "display dialog \"{}\" buttons {{\"OK\"}} default button \"OK\" with title \"About\"",
            message
        );
        
        let _ = Command::new("osascript")
            .arg("-e")
            .arg(&script)
            .spawn();
    }
}

fn toggle_dock_icon(show: bool, state: &AppState) -> Result<(), Box<dyn std::error::Error>> {
    // Save the preference
    let mut config = state.config.lock().unwrap();
    config.show_dock_icon = show;
    drop(config);
    state.save_config()?;
    
    #[cfg(target_os = "macos")]
    {
        use std::process::Command;
        
        // Get the app bundle path
        let current_exe = std::env::current_exe()?;
        let bundle_path = if current_exe.to_string_lossy().contains(".app/Contents/MacOS") {
            // Running from app bundle - find the .app
            current_exe.parent()
                .and_then(|p| p.parent())
                .and_then(|p| p.parent())
        } else {
            None
        };
        
        // Update Info.plist if running from bundle
        if let Some(bundle) = bundle_path {
            let plist_path = bundle.join("Contents/Info.plist");
            if plist_path.exists() {
                // Use PlistBuddy to update LSUIElement
                let value = if show { "false" } else { "true" };
                let _ = Command::new("/usr/libexec/PlistBuddy")
                    .args(&["-c", &format!("Set :LSUIElement {}", value), plist_path.to_str().unwrap()])
                    .output();
            }
        }
        
        // Notify user that restart is required
        let message = if show {
            "Dock icon will appear after restarting the app.\n\nQuit and relaunch Lapsus Control?"
        } else {
            "Dock icon will be hidden after restarting the app.\n\nQuit and relaunch Lapsus Control?"
        };
        
        let result = Command::new("osascript")
            .arg("-e")
            .arg(format!(
                "display dialog \"{}\" buttons {{\"Later\", \"Restart Now\"}} default button \"Restart Now\" with title \"Restart Required\"",
                message
            ))
            .output()?;
        
        // Check if user clicked "Restart Now"
        let output = String::from_utf8_lossy(&result.stdout);
        if output.contains("Restart Now") {
            // Relaunch the app
            let current_exe = std::env::current_exe()?;
            Command::new("open")
                .arg("-n")
                .arg(bundle_path.unwrap())
                .spawn()?;
            std::process::exit(0);
        }
        
        Ok(())
    }
    
    #[cfg(not(target_os = "macos"))]
    Ok(())
}

fn show_error_dialog(message: &str) {
    #[cfg(target_os = "macos")]
    {
        use std::process::Command;
        let escaped = message.replace("\"", "\\\"").replace("\n", "\\n");
        let script = format!(
            "display dialog \"{}\" buttons {{\"OK\"}} default button \"OK\" with title \"Error\" with icon stop",
            escaped
        );
        let _ = Command::new("osascript")
            .arg("-e")
            .arg(&script)
            .spawn();
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize app state
    let state = match AppState::new() {
        Ok(s) => s,
        Err(e) => {
            show_error_dialog(&format!("Failed to initialize app: {}", e));
            return Err(e);
        }
    };

    // Set dock icon visibility based on config
    #[cfg(target_os = "macos")]
    {
        let config = state.config.lock().unwrap();
        let _show_dock = config.show_dock_icon;
        drop(config);
        
        // Note: NSApplication activation policy is set via Info.plist LSUIElement=true
        // We can't change it at runtime without restart, so we just store the preference
        // The actual change happens on next launch via the toggle_dock_icon function
    }

    // Check if lapsus_rust exists
    if !state.lapsus_path.exists() {
        show_error_dialog(&format!(
            "lapsus_rust not found at: {:?}\n\nPlease ensure lapsus_rust is in the correct location.",
            state.lapsus_path
        ));
    }

    // Create event loop
    let event_loop = tao::event_loop::EventLoop::new();

    // Build initial menu
    let menu = build_menu(&state)?;
    
    // Create tray icon
    let tray_icon = TrayIconBuilder::new()
        .with_menu(Box::new(menu))
        .with_tooltip(APP_NAME)
        .with_icon(state.get_current_icon())
        .build()?;

    // Clone state for event handling
    let state_clone = state.clone();
    let tray_icon_handle = Arc::new(Mutex::new(tray_icon));
    let tray_clone = tray_icon_handle.clone();

    // Menu event handler
    let menu_channel = MenuEvent::receiver();
    
    // Timer for polling process status
    let mut last_check = std::time::Instant::now();
    let mut last_running_state = state.is_lapsus_running();

    event_loop.run(move |_event, _, control_flow| {
        *control_flow = tao::event_loop::ControlFlow::WaitUntil(
            std::time::Instant::now() + Duration::from_millis(100)
        );

        // Check for menu events
        if let Ok(event) = menu_channel.try_recv() {
            let menu_id = event.id.0.as_str();
            
            match menu_id {
                "enable" => {
                    if let Err(e) = state_clone.start_lapsus() {
                        show_error_dialog(&format!("Failed to start lapsus_rust: {}", e));
                    } else {
                        // Update menu and icon
                        if let Ok(new_menu) = build_menu(&state_clone) {
                            let tray = tray_clone.lock().unwrap();
                            let _ = tray.set_menu(Some(Box::new(new_menu)));
                            let _ = tray.set_icon(Some(state_clone.icon_enabled.clone()));
                        }
                    }
                }
                "disable" => {
                    if let Err(e) = state_clone.stop_lapsus() {
                        show_error_dialog(&format!("Failed to stop lapsus_rust: {}", e));
                    } else {
                        // Update menu and icon
                        if let Ok(new_menu) = build_menu(&state_clone) {
                            let tray = tray_clone.lock().unwrap();
                            let _ = tray.set_menu(Some(Box::new(new_menu)));
                            let _ = tray.set_icon(Some(state_clone.icon_disabled.clone()));
                        }
                    }
                }
                "start_at_login" => {
                    let config = state_clone.config.lock().unwrap();
                    let current = config.start_at_login;
                    drop(config);
                    
                    if let Err(e) = state_clone.toggle_auto_launch(!current) {
                        show_error_dialog(&format!("Failed to toggle auto-launch: {}", e));
                    } else {
                        // Update menu to reflect new state
                        if let Ok(new_menu) = build_menu(&state_clone) {
                            let tray = tray_clone.lock().unwrap();
                            let _ = tray.set_menu(Some(Box::new(new_menu)));
                        }
                    }
                }
                "show_dock_icon" => {
                    let config = state_clone.config.lock().unwrap();
                    let current = config.show_dock_icon;
                    drop(config);
                    
                    if let Err(e) = toggle_dock_icon(!current, &state_clone) {
                        show_error_dialog(&format!("Failed to toggle dock icon: {}", e));
                    } else {
                        // Update menu to reflect new state
                        if let Ok(new_menu) = build_menu(&state_clone) {
                            let tray = tray_clone.lock().unwrap();
                            let _ = tray.set_menu(Some(Box::new(new_menu)));
                        }
                    }
                }
                "about" => {
                    show_about_dialog();
                }
                "quit" => {
                    // Check if lapsus_rust is running and stop it
                    if state_clone.is_lapsus_running() {
                        let _ = state_clone.stop_lapsus();
                    }
                    *control_flow = tao::event_loop::ControlFlow::Exit;
                }
                _ => {
                    eprintln!("Unknown menu item: {}", menu_id);
                }
            }
        }

        // Poll for process state changes every 2 seconds
        if last_check.elapsed() > Duration::from_secs(2) {
            let is_running = state_clone.is_lapsus_running();
            
            if is_running != last_running_state {
                // State changed, update UI
                if let Ok(new_menu) = build_menu(&state_clone) {
                    let tray = tray_clone.lock().unwrap();
                    let _ = tray.set_menu(Some(Box::new(new_menu)));
                    let _ = tray.set_icon(Some(state_clone.get_current_icon()));
                }
                last_running_state = is_running;
            }
            
            last_check = std::time::Instant::now();
        }
    });
}
