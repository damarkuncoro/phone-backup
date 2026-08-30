use std::process::Command;

pub struct AdbCommandBuilder<'a> {
    serial: Option<&'a str>,
    args: Vec<&'a str>,
}

impl<'a> AdbCommandBuilder<'a> {
    pub fn new() -> Self {
        Self { serial: None, args: Vec::new() }
    }

    pub fn on_device(mut self, serial: &'a str) -> Self {
        self.serial = Some(serial);
        self
    }

    pub fn arg(mut self, arg: &'a str) -> Self {
        self.args.push(arg);
        self
    }

    pub fn shell(mut self, script: &'a str) -> Self {
        self.args.push("shell");
        self.args.push(script);
        self
    }

    pub fn build(self, adb_path: &str) -> Command {
        let mut cmd = Command::new(adb_path);
        if let Some(s) = self.serial {
            cmd.arg("-s").arg(s);
        }
        cmd.args(&self.args);
        cmd
    }
}
