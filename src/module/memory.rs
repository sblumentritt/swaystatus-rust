use swaystatus::Module;

use lazy_static::lazy_static;
use regex::Regex;
use std::fs;

pub struct Memory {
    total: u32,
    free: u32,
    buffers: u32,
    cached: u32,
    slab: u32,
    used: u32,
}

impl Memory {
    const MEMINFO_PATH: &'static str = "/proc/meminfo";

    pub fn new() -> Self {
        Memory {
            total: 0,
            free: 0,
            buffers: 0,
            cached: 0,
            slab: 0,
            used: 0,
        }
    }
}

impl Module for Memory {
    fn value(&self) -> String {
        format!("{} MB/{} MB", self.used, self.total)
    }

    fn update(&mut self) {
        lazy_static! {
            static ref MEMTOTAL_RE: Regex = Regex::new(r"\bMemTotal:\s*(\d*)").unwrap();
            static ref MEMFREE_RE: Regex = Regex::new(r"\bMemFree:\s*(\d*)").unwrap();
            static ref BUFFERS_RE: Regex = Regex::new(r"\bBuffers:\s*(\d*)").unwrap();
            static ref CACHED_RE: Regex = Regex::new(r"\bCached:\s*(\d*)").unwrap();
            static ref SLAB_RE: Regex = Regex::new(r"\bSlab:\s*(\d*)").unwrap();
        }

        // TODO: replace unwrap() calls with correct error handling
        let meminfo = fs::read_to_string(Memory::MEMINFO_PATH).unwrap();

        // find the values with the help of the regexes and convert them from KB to MB
        for cap in MEMTOTAL_RE.captures_iter(meminfo.as_str()) {
            self.total = cap[1].parse::<u32>().unwrap() / 1024;
        }
        for cap in MEMFREE_RE.captures_iter(meminfo.as_str()) {
            self.free = cap[1].parse::<u32>().unwrap() / 1024;
        }
        for cap in BUFFERS_RE.captures_iter(meminfo.as_str()) {
            self.buffers = cap[1].parse::<u32>().unwrap() / 1024;
        }
        for cap in CACHED_RE.captures_iter(meminfo.as_str()) {
            self.cached = cap[1].parse::<u32>().unwrap() / 1024;
        }
        for cap in SLAB_RE.captures_iter(meminfo.as_str()) {
            self.slab = cap[1].parse::<u32>().unwrap() / 1024;
        }

        // calculate the memory usage according to:
        // https://access.redhat.com/solutions/406773
        self.used = self.total - self.free - self.buffers - self.cached - self.slab;
    }
}
