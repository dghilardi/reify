use crate::processor::ReifyProcessor;

pub fn process_template<P: ReifyProcessor>(src_path: &str, dst_path: &str, processor: P) -> anyhow::Result<()> {
    let template = std::fs::read_to_string(src_path)?;
    let rendered = processor.render(&template)?;
    std::fs::write(dst_path, rendered)?;
    Ok(())
}