// Copyright 2016 PingCAP, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// See the License for the specific language governing permissions and
// limitations under the License.

use sys_info;

pub const DEFAULT_DATA_DIR: &'static str = "";
const DEFAULT_GC_RATIO_THRESHOLD: f64 = 1.1;
const DEFAULT_SCHED_CAPACITY: usize = 10240;
const DEFAULT_SCHED_MSG_PER_TICK: usize = 1024;
const DEFAULT_SCHED_CONCURRENCY: usize = 102400;
const DEFAULT_SCHED_TOO_BUSY_THRESHOLD: usize = 1000;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
#[serde(rename_all = "kebab-case")]
pub struct Config {
    pub data_dir: String,
    pub gc_ratio_threshold: f64,
    pub scheduler_notify_capacity: usize,
    pub scheduler_message_per_tick: usize,
    pub scheduler_concurrency: usize,
    pub scheduler_worker_pool_size: usize,
    pub scheduler_too_busy_threshold: usize,
}

impl Default for Config {
    fn default() -> Config {
        let total_cpu = sys_info::cpu_num().unwrap();
        Config {
            data_dir: DEFAULT_DATA_DIR.to_owned(),
            gc_ratio_threshold: DEFAULT_GC_RATIO_THRESHOLD,
            scheduler_notify_capacity: DEFAULT_SCHED_CAPACITY,
            scheduler_message_per_tick: DEFAULT_SCHED_MSG_PER_TICK,
            scheduler_concurrency: DEFAULT_SCHED_CONCURRENCY,
            scheduler_worker_pool_size: if total_cpu >= 16 { 8 } else { 4 },
            scheduler_too_busy_threshold: DEFAULT_SCHED_TOO_BUSY_THRESHOLD,
        }
    }
}
