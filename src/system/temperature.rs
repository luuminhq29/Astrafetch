use serde::Serialize;
use std::{fs, path::Path};

#[derive(Debug, Clone, Serialize)]
pub struct TemperatureInfo {
    pub cpu_c: Option<f64>,
    pub gpu_c: Option<f64>,
}

pub fn collect() -> TemperatureInfo {
    let mut temps = Vec::new();

    if let Ok(entries) = fs::read_dir("/sys/class/thermal") {
        for entry in entries.flatten() {
            let path = entry.path();

            let is_thermal_zone = path
                .file_name()
                .and_then(|name| name.to_str())
                .map(|name| name.starts_with("thermal_zone"))
                .unwrap_or(false);

            if !is_thermal_zone {
                continue;
            }

            if let Ok(value) = fs::read_to_string(path.join("temp")) {
                if let Ok(value) = value.trim().parse::<f64>() {
                    temps.push(value / 1000.0);
                }
            }
        }
    }

    TemperatureInfo {
        cpu_c: temps.into_iter().find(|temp| *temp > 0.0 && *temp < 150.0),
        gpu_c: gpu_temp(),
    }
}

fn gpu_temp() -> Option<f64> {
    let root = Path::new("/sys/class/drm");

    for entry in fs::read_dir(root).ok()?.flatten() {
        let hwmon_path = entry.path().join("device/hwmon");

        if let Ok(hwmons) = fs::read_dir(hwmon_path) {
            for hwmon in hwmons.flatten() {
                if let Ok(entries) = fs::read_dir(hwmon.path()) {
                    for entry in entries.flatten() {
                        let name = entry.file_name().to_string_lossy().into_owned();

                        if name.starts_with("temp") && name.ends_with("_input") {
                            if let Ok(value) = fs::read_to_string(entry.path()) {
                                if let Ok(value) = value.trim().parse::<f64>() {
                                    return Some(value / 1000.0);
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    None
}
