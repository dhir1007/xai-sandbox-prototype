use wasmtime::*;

#[derive(Debug, Clone)]
pub struct SandboxState {
    pub max_memory_bytes: usize, // Max memory in bytes.
}

impl ResourceLimiter for SandboxState {
    fn memory_growing(&mut self, _current: usize, desired: usize, _max: Option<usize>) -> anyhow::Result<bool> {
        Ok(desired <= self.max_memory_bytes) // if the desired memory is less than the max memory, return true.
    }

    fn table_growing(&mut self, _current: usize, desired: usize, _max: Option<usize>) -> anyhow::Result<bool> {
        Ok(desired <= 1000) // if the desired table is less than 1000, return true.
    }
}