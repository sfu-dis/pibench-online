#[derive(Serialize, Deserialize, Debug)]
pub struct InstanceInfo {
    pub url: String,
    pub has_pm: bool,
    pub cpu_info: CPUInfo,
    pub wrappers: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BenchmarkBasicParam {
    pub backend: String,
    pub wrapper: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PiBenchParam {
    pub thread_cnt: i32,
    pub load_cnt: i32,
    pub op_cnt: i32,
    pub sample_time: i32,
    pub read: f32,
    pub insert: f32,
    pub update: f32,
    pub delete: f32,
    pub latency_sampling: f32,
    pub use_pcm: bool,
    pub skip_load: bool,
    pub scan_size: i32,
    pub key_size: i32,
    pub value_size: i32,
    pub pool_size: i64,
    pub pool_path: String,
    pub key_prefix: String,
    pub distribution: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BenchmarkParam {
    pub basic: BenchmarkBasicParam,
    pub params: PiBenchParam,
    pub env: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BenchmarkResponse {
    pub result: String,
    pub timestamp: i64,
    pub pid: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BenchmarkEnv {
    pub time: String,
    pub cpu: String,
    pub cpu_cache: String,
    pub kernel: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeleteWrapper {
    pub wrapper_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CPUInfo {
    pub cpu_cnt: i32,
    pub threads_per_core: i32,
    pub socket_cnt: i32,
    pub l1d_cache: String,
    pub l1i_cache: String,
    pub l2_cache: String,
    pub l3_cache: String,
    pub architecture: String,
    pub model_name: String,
    pub cpu_flags: String,
}
