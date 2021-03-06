use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::File;
use std::io;
use std::io::prelude::*;

const SETTINGS_FILE: &str = "/usr/share/razercontrol/daemon.json";
const EFFECTS_FILE: &str = "/usr/share/razercontrol/effects.json";

#[derive(Serialize, Deserialize)]
pub struct Configuration {
    pub power_mode: u8,
    pub cpu_boost: u8,
    pub gpu_boost: u8,
    pub fan_rpm: i32,
    pub brightness: u8,
}

impl Configuration {
    pub fn new() -> Configuration {
        return Configuration {
            power_mode: 0,
            cpu_boost: 0,
            gpu_boost: 0,
            fan_rpm: 0,
            brightness: 128,
        };
    }

    pub fn write_to_file(&mut self) -> io::Result<()> {
        let j: String = serde_json::to_string_pretty(&self)?;
        File::create(SETTINGS_FILE)?.write_all(j.as_bytes())?;
        Ok(())
    }

    pub fn read_from_config() -> io::Result<Configuration> {
        let str = fs::read_to_string(SETTINGS_FILE)?;
        let res: Configuration = serde_json::from_str(str.as_str())?;
        Ok(res)
    }

    pub fn write_effects_save(json: serde_json::Value) -> io::Result<()> {
        let j: String = serde_json::to_string_pretty(&json)?;
        File::create(EFFECTS_FILE)?.write_all(j.as_bytes())?;
        Ok(())
    }

    pub fn read_effects_file() -> io::Result<serde_json::Value> {
        let str = fs::read_to_string(EFFECTS_FILE)?;
        let res: serde_json::Value = serde_json::from_str(str.as_str())?;
        Ok(res)
    }
}
