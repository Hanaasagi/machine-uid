use std::fs::File;
use std::io::prelude::*;
use std::error::Error;

fn read_file(file_path: &str) -> Result<String, Box<Error>> {
    let mut fd = File::open(file_path)?;
    let mut content = String::new();
    fd.read_to_string(&mut content)?;
    Ok(content.trim().to_string())
}

#[cfg(target_os="linux")]
pub mod machine_id {
    use ::read_file;
    use std::error::Error;

    // dbusPath is the default path for dbus machine id.
    const DBUS_PATH: &str = "/var/lib/dbus/machine-id";
    // or when not found (e.g. Fedora 20)
    const DBUS_PATH_ETC: &str = "/etc/machine-id";

    pub fn get_machine_id() -> Result<String, Box<Error>> {
        match read_file(DBUS_PATH) {
            Ok(machine_id) => Ok(machine_id),
            Err(_) => Ok(read_file(DBUS_PATH_ETC)?)
        }

    }
}

#[cfg(any(target_os="freebsd", target_os="dragonfly", target_os="openbsd", target_os="netbsd"))]
pub mod machine_id {
    use ::read_file;
    const HOST_ID_PATH: &str = "/etc/hostid";

    pub fn get_machine_id() -> Result<String, Box<Error>> {
        match read_file(HOST_ID_PATH) {
            Ok(machine_id) => return Ok(machine_id),
            Err(_) => Ok(read_from_kenv()?)
        }
    }

    fn read_from_kenv() -> Result<String, Box<Error>> {
        let output = Command::new("kenv")
            .args(&["-q", "smbios.system.uuid"])
            .output()?;
        let content = String::from_utf8_lossy(&output.stdout);
        Ok(content.trim().to_string())
    }

}

#[cfg(target_os="macos")]
mod machine_id {
    use std::process::Command;
    use std::error::Error;

    // machineID returns the uuid returned by `ioreg -rd1 -c IOPlatformExpertDevice`.
    pub fn get_machine_id() -> Result<String, Box<Error>> {
       let output = Command::new("ioreg")
           .args(&["-rd1", "-c", "IOPlatformExpertDevice"])
           .output()?;
       let content = String::from_utf8_lossy(&output.stdout);
       extract_id(&content)
    }

    fn extract_id(content: &str) -> Result<String, Box<Error>> {
       let lines = content.split("\n");
       for line in lines {
           if line.contains("IOPlatformUUID") {
               let k: Vec<&str> = line.rsplitn(2, "=").collect();
               let id = k[0].trim_matches(|c: char| c == '"' || c.is_whitespace());
               return Ok(id.to_string());
           }
       }
       Err(From::from("No matching IOPlatformUUID in `ioreg -rd1 -c IOPlatformExpertDevice` command."))
    }
}

pub use machine_id::get_machine_id as get;
