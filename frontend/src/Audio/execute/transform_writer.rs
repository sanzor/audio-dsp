pub trait TransformWriter {
    fn update(&self, executor: &GraphExecutor, instances: Vec<Box<dyn WasmInstance>>) -> Result<(), String>;
}
