use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Note: `tun` is disabled so this integration test does not require TUN privileges.
    // Ports are chosen to avoid clashing with a typical local Clash/mihomo (17890/9090, etc.).
    let yaml = r#"
port: 37890
mixed-port: 37890
allow-lan: false
external-controller: 127.0.0.1:39090
secret: ""

tun:
  enable: false

proxies:
  - name: LOCAL_DIRECT
    type: direct
    udp: true

proxy-groups:
  - name: GLOBAL
    type: select
    proxies: [LOCAL_DIRECT]

rules:
  - MATCH,GLOBAL
"#;

    let _core = mihomo::Mihomo::start(yaml)?;

    // Give listeners a moment to come up.
    std::thread::sleep(Duration::from_millis(500));

    let body = reqwest::blocking::get("http://127.0.0.1:39090/version")?.text()?;
    println!("mihomo /version:\n{}", body);
    Ok(())
}
