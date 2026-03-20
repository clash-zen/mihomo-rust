use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Minimal config for in-process start.
    // Note: `tun` is disabled so the example does not require TUN privileges.
    let yaml = r#"
port: 7890
mixed-port: 7890
allow-lan: false
external-controller: 127.0.0.1:9090
secret: ""

tun:
  enable: false

proxies:
  - name: DIRECT
    type: direct
    udp: true

proxy-groups:
  - name: GLOBAL
    type: select
    proxies: [DIRECT]

rules:
  - MATCH,GLOBAL
"#;

    let _core = mihomo::Mihomo::start(yaml)?;

    // Give listeners a moment to come up.
    std::thread::sleep(Duration::from_millis(500));

    let body = reqwest::blocking::get("http://127.0.0.1:9090/version")?.text()?;
    println!("mihomo /version:\n{}", body);
    Ok(())
}
