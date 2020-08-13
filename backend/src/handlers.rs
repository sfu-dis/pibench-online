use crate::cpu_info;
use crate::types::*;
use chrono::{DateTime, Utc};
use rocket_contrib::json::{Json, JsonValue};
extern crate serde_json;
extern crate subprocess;
use rocket::http::ContentType;
use std::collections::HashMap;
use std::fs::{self, File};
use std::sync::Mutex;
use std::{env, io, path};

extern crate rocket_multipart_form_data;
use rocket_multipart_form_data::{
    MultipartFormData, MultipartFormDataField, MultipartFormDataOptions, RawField,
};
use std::io::Write;

lazy_static! {
    pub static ref PROCESS: Mutex<HashMap<u32, subprocess::Popen>> = {
        let m = HashMap::new();
        Mutex::new(m)
    };
    pub static ref BACKEND_URL: String = match env::var("PIBENCH_URL") {
        Ok(val) => val,
        Err(_e) => "http://home.haoxp.xyz:7000".to_string(),
    };
    pub static ref PIBENCH_BIN: String = match env::var("PIBENCH_BIN") {
        Ok(val) => val,
        Err(_e) => "./target/pibench_build/build/src/PiBench".to_string(),
    };
    pub static ref WRAPPER_DIR: String = match env::var("PIBENCH_WRAPPER") {
        Ok(val) => val,
        Err(_e) => "./wrappers".to_string(),
    };
}

#[get("/info")]
pub fn get_info() -> Json<InstanceInfo> {
    let mut entries = fs::read_dir(WRAPPER_DIR.to_string())
        .unwrap()
        .map(|res| res.map(|e| e.path().file_name().unwrap().to_str().unwrap().to_string()))
        .filter(|v| v.is_ok() && v.as_ref().unwrap().as_bytes()[0] != '.' as u8)
        .collect::<Result<Vec<_>, io::Error>>()
        .unwrap();
    entries.sort();

    let has_pm = match fs::read_dir("/mnt/pmem0") {
        Ok(_val) => true,
        Err(_) => false,
    };

    let cpu_info = cpu_info::get_cpu_info().unwrap();

    Json(InstanceInfo {
        url: BACKEND_URL.to_string(),
        has_pm,
        cpu_info,
        wrappers: entries,
    })
}

#[post("/upload_wrapper", format = "multipart/form-data", data = "<data>")]
pub fn upload_wrapper(content_type: &ContentType, data: rocket::Data) {
    let mut options = MultipartFormDataOptions::new();
    options
        .allowed_fields
        .push(MultipartFormDataField::raw("wrapper").size_limit(32 * 1024 * 1024));
    let mut multipart_form_data = match MultipartFormData::parse(content_type, data, options) {
        Ok(multipart_form_data) => multipart_form_data,
        Err(err) => match err {
            _ => panic!("{:?}", err),
        },
    };

    let wrapper = multipart_form_data.raw.remove("wrapper");
    match wrapper {
        Some(wrapper) => match wrapper {
            RawField::Single(raw) => {
                let file_name = raw.file_name.unwrap_or("invalid_name".to_string());
                let upload_path = path::Path::new(&WRAPPER_DIR.to_string()).join(&file_name);
                let mut buffer = File::create(upload_path).unwrap();
                match buffer.write(&raw.raw) {
                    Ok(_) => println!("file {} saved!", &file_name),
                    Err(_) => println!("err occurred"),
                };
            }
            RawField::Multiple(_) => unreachable!(),
        },
        None => println!("Please input a file."),
    }
}

#[post("/remove_wrapper", format = "json", data = "<payload>")]
pub fn remove_wrapper(payload: Json<DeleteWrapper>) -> JsonValue {
    let removed =
        fs::remove_file(path::Path::new(&WRAPPER_DIR.to_string()).join(&payload.wrapper_name));
    match removed {
        Ok(_) => json!({"result":"ok"}),
        Err(_) => json!({"result":"failed"}),
    }
}

#[post("/benchmark", format = "json", data = "<benchmark>")]
pub fn start_benchmark(benchmark: Json<BenchmarkParam>) -> Json<BenchmarkResponse> {
    let pibench_param = &benchmark.params;
    let process = subprocess::Popen::create(
        &[
            PIBENCH_BIN.to_string(),
            path::Path::new(&WRAPPER_DIR.to_string())
                .join(&benchmark.basic.wrapper)
                .to_str()
                .unwrap()
                .to_string(),
            "-i".to_string(),
            pibench_param.insert.to_string(),
            "-r".to_string(),
            pibench_param.read.to_string(),
            "-u".to_string(),
            pibench_param.update.to_string(),
            "-d".to_string(),
            pibench_param.delete.to_string(),
            "-n".to_string(),
            pibench_param.load_cnt.to_string(),
            "-p".to_string(),
            pibench_param.op_cnt.to_string(),
            "-t".to_string(),
            pibench_param.thread_cnt.to_string(),
            "-f".to_string(),
            pibench_param.key_prefix.clone(),
            "-k".to_string(),
            pibench_param.key_size.to_string(),
            "-v".to_string(),
            pibench_param.value_size.to_string(),
            "--sampling_ms".to_string(),
            pibench_param.sample_time.to_string(),
            "--latency_sampling".to_string(),
            pibench_param.latency_sampling.to_string(),
            "--distribution".to_string(),
            pibench_param.distribution.clone(),
            "--pcm=false".to_string(),
            // &pibench_param.use_pcm.to_string(),
            "--pool_size".to_string(),
            pibench_param.pool_size.to_string(),
            "--pool_path".to_string(),
            pibench_param.pool_path.clone(),
        ],
        subprocess::PopenConfig {
            stdout: subprocess::Redirection::Pipe,
            ..Default::default()
        },
    )
    .unwrap();
    let process_id = process.pid().unwrap();
    PROCESS.lock().unwrap().entry(process_id).or_insert(process);

    let now: DateTime<Utc> = Utc::now();
    Json(BenchmarkResponse {
        result: "Ok".to_string(),
        timestamp: now.timestamp(),
        pid: process_id,
    })
}

#[get("/status?<pid>")]
pub fn get_status(pid: u32) -> JsonValue {
    let mut process_map = PROCESS.lock().unwrap();
    let process = process_map.get_mut(&pid).unwrap();
    if let Some(_status) = process.poll() {
        let (out, _err) = process.communicate(None).unwrap();
        return json!({
            "status": "finished",
            "data": out
        });
    } else {
        return json!({
            "status":"running"
        });
    }
}
