use crate::types::*;
use std::collections::HashMap;
use std::process::Command;
use std::str;

pub fn get_cpu_info() -> Result<CPUInfo, &'static str> {
    let output = match Command::new("lscpu").output() {
        Ok(output) => output,
        Err(_e) => return Err("failed to execute lscpu"),
    };

    let output = match str::from_utf8(&output.stdout) {
        Ok(output) => output,
        Err(_e) => return Err("failed to parse output"),
    };

    let mut result_map: HashMap<&str, &str> = HashMap::new();

    for line in output.lines() {
        let kv: Vec<&str> = line.split(":").collect();
        result_map.insert(kv[0].trim(), kv[1].trim());
    }

    let cpu_info = CPUInfo {
        cpu_cnt: result_map.get("CPU(s)").unwrap_or(&"0").parse().unwrap(),
        threads_per_core: result_map
            .get("Thread(s) per core")
            .unwrap_or(&"0")
            .parse()
            .unwrap(),
        socket_cnt: result_map.get("Socket(s)").unwrap_or(&"0").parse().unwrap(),
        l1d_cache: result_map
            .get("L1d cache")
            .unwrap_or(&"unknown")
            .to_string(),
        l1i_cache: result_map
            .get("L1i cache")
            .unwrap_or(&"unknown")
            .to_string(),
        l2_cache: result_map.get("L2 cache").unwrap_or(&"unknown").to_string(),
        l3_cache: result_map.get("L3 cache").unwrap_or(&"unknown").to_string(),
        architecture: result_map
            .get("Architecture")
            .unwrap_or(&"unknown")
            .to_string(),
        model_name: result_map
            .get("Model name")
            .unwrap_or(&"unknown")
            .to_string(),
        cpu_flags: result_map.get("Flags").unwrap_or(&"unknown").to_string(),
    };

    Ok(cpu_info)
}
