use crate::constants::BUS_SOURCES_PREFIX;

pub mod interface;
pub mod polkit;
#[cfg(test)]
pub mod polkit_test;

pub fn create_source_device_path(device_id: &str) -> String {
    let sanitized_device_id = device_id
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { '_' })
        .collect::<String>();
    format!("{}/{}", BUS_SOURCES_PREFIX, sanitized_device_id)
}
