use std::env; // For getting commandline arguments and reading Environment
              // Variables.
extern crate uptime_lib;
use std::error::Error;
use std::process::Command; // For exit with a code.
use sysinfo::{System, SystemExt};
pub fn get_sys_info() -> Result<(String, String, String), Box<dyn Error>> {
    let sys = System::new_all();
    // sys.refresh_system() // having to refresh in this case has a pointless runtime cost.;
    let os_name = format!("{} {}", sys.name().unwrap(), sys.os_version().unwrap());
    let host_name = sys.host_name().unwrap();
    let kernel = sys.kernel_version().unwrap();
    Ok((os_name, kernel, host_name))
}

pub fn get_shell_name() -> String {
    // Read the value of the Environment Variable, `SHELL`
    // to obtain the current shell name.
    let shell_var = "SHELL";
    match env::var(shell_var) {
        Ok(mut val) => {
            val = val.replace('/', " "); // Replace all the forward slashes
                                         // with a space.
            val.split(' ').last().unwrap().to_string() // Split the string
                                                       // based on the spaces
                                                       // and get the last word.
        }
        Err(_) => "Unknown".to_string(), // If the Environment variable is
                                         // not read, return "Unknown".
    }
}
pub fn get_sys_uptime() -> String {
    match uptime_lib::get() {
        Ok(uptime) => {
            let uptime_seconds = uptime.as_secs();
            let days = uptime_seconds / (24 * 3600);
            let hours = (uptime_seconds % (24 * 3600)) / 3600;
            let minutes = (uptime_seconds % 3600) / 60;
            let mut form_days: String = "".to_string();
            let mut form_hours: String = "".to_string();
            if days >= 1 {
                form_days = format!("{} days, ", days);
            }
            if hours >= 1 {
                form_hours = format!("{} hrs, ", hours);
            }
            format!("{}{}{} min", form_days, form_hours, minutes)
        }
        Err(err) => {
            eprintln!("uptime: {}", err);
            std::process::exit(1);
        }
    }
}
pub fn get_session_name() -> String {
    // First check `DESKTOP_SESSION`.
    // Read the value of the Environment Variable, `DESKTOP_SESSION`
    // to obtain the name of the DE(Desktop Environment) or WM(Window Manager).
    let session_name = "DESKTOP_SESSION";
    // If that fails, assign `wm_name` to `Unknown`
    let wm_name = env::var(session_name).unwrap_or_else(|_| "Unknown".to_string());
    // If there was an error, in reading `DESKTOP_SESSION`, or if it is empty,
    // try reading `XDG_SESSION_DESKTOP`.
    if wm_name.is_empty() || wm_name == *"Unknown" {
        let session_name = "XDG_SESSION_DESKTOP";
        let wm_name = env::var(session_name).unwrap_or_else(|_| "Unknown".to_string());
        // If there was an error, in reading `XDG_SESSION_DESKTOP`, or if it is
        // empty, try reading `XDG_CURRENT_DESKTOP`.
        if wm_name.is_empty() || wm_name == *"Unknown" {
            let session_name = "XDG_CURRENT_DESKTOP";
            let wm_name = env::var(session_name).unwrap_or_else(|_| "Unknown".to_string());
            // If there was an error, in reading `XDG_CURRENT_DESKTOP`, fall back
            // to reading `_NET_WM_NAME` using `xprop`.
            if wm_name.is_empty() || wm_name == *"Unknown" {
                // Now, we try looking at `_NET_WM_NAME`, by using `xprop`.
                let xprop_id = Command::new("xprop")
                    .args(["-root", "-notype", "_NET_SUPPORTING_WM_CHECK"])
                    .output();
                // If the above command ran successfully, assign its output to `xprop_id`.
                let xprop_id = match xprop_id {
                    Ok(x) => String::from_utf8(x.stdout).unwrap(),
                    Err(_) => "Unknown".to_string(),
                };
                // Extract the ID
                let xprop_id = xprop_id.split(' ').last().unwrap();
                // Call `xprop` again, but now by passing in the ID, we just found.
                let mut wm_name = match Command::new("xprop")
                    .args(["-id", xprop_id, "-notype"])
                    .output()
                {
                    Ok(x) => String::from_utf8(x.stdout).unwrap(),
                    Err(_) => "Unknown".to_string(),
                };
                // Now, from the output, of the above call, we look for `_NET_WM_NAME`.
                for line in wm_name.lines() {
                    if line.contains("_NET_WM_NAME") {
                        wm_name = line
                            .split('=')
                            .last()
                            .unwrap()
                            .to_string()
                            .replace(['"', ' '], ""); // Remove double-quotes.
                                               // present between the `_NET_WM_NAME`
                                               // and it's value, after the `=` sign.
                        return wm_name;
                    }
                }
                // If all else fails, return "Unknown".
                String::from("Unknown")
            } else {
                wm_name
            }
        } else {
            wm_name
        }
    } else {
        wm_name
    }
}
// Add some tests, for testing the `get_session_name()` function.
#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    #[test]
    fn fallback_session() {
        // Remove all the checked Environment variables.
        let env_var_1 = "DESKTOP_SESSION";
        env::remove_var(env_var_1);
        let env_var_2 = "XDG_CURRENT_DESKTOP";
        env::remove_var(env_var_2);
        let env_var_3 = "XDG_SESSION_DESKTOP";
        env::remove_var(env_var_3);
        let wm_name = get_session_name();
        assert_eq!(wm_name, "wlrootswm");
    }
    #[test]
    fn xdg_current() {
        // Remove all the checked Environment variables.
        let env_var_1 = "DESKTOP_SESSION";
        env::remove_var(env_var_1);
        let env_var_2 = "XDG_CURRENT_DESKTOP";
        env::remove_var(env_var_2);
        let env_var_3 = "XDG_SESSION_DESKTOP";
        env::remove_var(env_var_3);
        // Set `XDG_SESSION_DESKTOP`
        let env_var = "XDG_CURRENT_DESKTOP";
        env::set_var(env_var, "Qtile");
        let wm_name = get_session_name();
        assert_eq!(wm_name, "Qtile");
    }
    #[test]
    fn xdg_session() {
        // Remove all the checked Environment variables.
        let env_var_1 = "DESKTOP_SESSION";
        env::remove_var(env_var_1);
        let env_var_2 = "XDG_CURRENT_DESKTOP";
        env::remove_var(env_var_2);
        let env_var_3 = "XDG_SESSION_DESKTOP";
        env::remove_var(env_var_3);
        // Set `XDG_SESSION_DESKTOP`
        let env_var = "XDG_SESSION_DESKTOP";
        env::set_var(env_var, "Testing");
        let wm_name = get_session_name();
        assert_eq!(wm_name, "Testing");
    }
}
