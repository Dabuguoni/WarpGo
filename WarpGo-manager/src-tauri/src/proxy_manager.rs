use anyhow::{Context, Result};
use std::process::{Child, Command, Stdio};
use std::path::PathBuf;

pub struct ProxyManager {
    mitmproxy_process: Option<Child>,
    port: u16,
}

impl ProxyManager {
    pub fn new() -> Self {
        Self {
            mitmproxy_process: None,
            port: 8080,
        }
    }

    pub fn start_mitmproxy(&mut self, script_path: PathBuf) -> Result<()> {
        if self.is_running() {
            return Ok(());
        }

        let child = Command::new("mitmdump")
            .arg("--listen-host")
            .arg("127.0.0.1")
            .arg("-p")
            .arg(self.port.to_string())
            .arg("-s")
            .arg(script_path)
            .arg("--set")
            .arg("confdir=~/.mitmproxy")
            .arg("--set")
            .arg("keep_host_header=true")
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .context("Failed to start mitmproxy")?;

        self.mitmproxy_process = Some(child);
        Ok(())
    }

    pub fn stop_mitmproxy(&mut self) -> Result<()> {
        if let Some(mut process) = self.mitmproxy_process.take() {
            process.kill()?;
            process.wait()?;
        }
        Ok(())
    }

    pub fn is_running(&self) -> bool {
        self.mitmproxy_process.is_some()
    }

    pub fn enable_system_proxy(&self) -> Result<()> {
        #[cfg(target_os = "windows")]
        {
            self.enable_proxy_windows()?;
        }

        #[cfg(target_os = "macos")]
        {
            self.enable_proxy_macos()?;
        }

        Ok(())
    }

    pub fn disable_system_proxy(&self) -> Result<()> {
        #[cfg(target_os = "windows")]
        {
            self.disable_proxy_windows()?;
        }

        #[cfg(target_os = "macos")]
        {
            self.disable_proxy_macos()?;
        }

        Ok(())
    }

    #[cfg(target_os = "windows")]
    fn enable_proxy_windows(&self) -> Result<()> {
        use winreg::enums::*;
        use winreg::RegKey;

        let hkcu = RegKey::predef(HKEY_CURRENT_USER);
        let internet_settings = hkcu
            .open_subkey_with_flags(
                "Software\\Microsoft\\Windows\\CurrentVersion\\Internet Settings",
                KEY_WRITE,
            )?;

        internet_settings.set_value("ProxyEnable", &1u32)?;
        internet_settings.set_value("ProxyServer", &format!("127.0.0.1:{}", self.port))?;

        // Refresh IE settings
        let _ = Command::new("rundll32.exe")
            .args(&["wininet.dll,InternetSetOption", "0", "37", "0", "0"])
            .output();

        Ok(())
    }

    #[cfg(target_os = "windows")]
    fn disable_proxy_windows(&self) -> Result<()> {
        use winreg::enums::*;
        use winreg::RegKey;

        let hkcu = RegKey::predef(HKEY_CURRENT_USER);
        let internet_settings = hkcu
            .open_subkey_with_flags(
                "Software\\Microsoft\\Windows\\CurrentVersion\\Internet Settings",
                KEY_WRITE,
            )?;

        internet_settings.set_value("ProxyEnable", &0u32)?;

        Ok(())
    }

    #[cfg(target_os = "macos")]
    fn enable_proxy_macos(&self) -> Result<()> {
        let service = self.get_network_service()?;

        // Create PAC file for selective proxying
        let pac_content = format!(
            r#"function FindProxyForURL(url, host) {{
    if (shExpMatch(host, "*.warp.dev") || 
        shExpMatch(host, "*warp.dev") ||
        shExpMatch(host, "*.dataplane.rudderstack.com") ||
        shExpMatch(host, "*dataplane.rudderstack.com")) {{
        return "PROXY 127.0.0.1:{}";
    }}
    return "DIRECT";
}}"#,
            self.port
        );

        let home = std::env::var("HOME")?;
        let pac_dir = PathBuf::from(&home).join(".warp_proxy");
        std::fs::create_dir_all(&pac_dir)?;
        let pac_file = pac_dir.join("warp_proxy.pac");
        std::fs::write(&pac_file, pac_content)?;

        let pac_url = format!("file://{}", pac_file.display());

        // Set auto proxy URL
        Command::new("networksetup")
            .args(&["-setautoproxyurl", &service, &pac_url])
            .output()?;

        // Enable auto proxy
        Command::new("networksetup")
            .args(&["-setautoproxystate", &service, "on"])
            .output()?;

        Ok(())
    }

    #[cfg(target_os = "macos")]
    fn disable_proxy_macos(&self) -> Result<()> {
        let service = self.get_network_service()?;

        // Disable auto proxy
        Command::new("networksetup")
            .args(&["-setautoproxystate", &service, "off"])
            .output()?;

        // Disable web proxy
        Command::new("networksetup")
            .args(&["-setwebproxystate", &service, "off"])
            .output()?;

        // Disable secure web proxy
        Command::new("networksetup")
            .args(&["-setsecurewebproxystate", &service, "off"])
            .output()?;

        // Clean up PAC file
        if let Ok(home) = std::env::var("HOME") {
            let pac_file = PathBuf::from(&home).join(".warp_proxy/warp_proxy.pac");
            let _ = std::fs::remove_file(pac_file);
        }

        Ok(())
    }

    #[cfg(target_os = "macos")]
    fn get_network_service(&self) -> Result<String> {
        let output = Command::new("networksetup")
            .arg("-listnetworkserviceorder")
            .output()?;

        let output_str = String::from_utf8(output.stdout)?;

        for line in output_str.lines() {
            if line.starts_with('(') && line.contains(')') {
                if let Some(service) = line.split(") ").nth(1) {
                    let service = service.trim();
                    if service != "Bluetooth PAN" && service != "Thunderbolt Bridge" {
                        return Ok(service.to_string());
                    }
                }
            }
        }

        anyhow::bail!("No suitable network service found")
    }
}

impl Drop for ProxyManager {
    fn drop(&mut self) {
        let _ = self.stop_mitmproxy();
    }
}
