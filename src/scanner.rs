use anyhow::Result;
use crate::process::Process;
use crate::memory;

#[derive(Debug)]
pub struct ScanResult {
    pub address: usize,
    pub value: String,
}

pub fn scan_exact(process: &Process, value: &str) -> Result<Vec<ScanResult>> {
    // Parse the value based on its format
    let parsed_value = if value.starts_with("0x") {
        // Hexadecimal
        u32::from_str_radix(&value[2..], 16)?
    } else if value.contains('.') {
        // Float (simplified)
        value.parse::<f32>()? as u32
    } else {
        // Decimal
        value.parse::<u32>()?
    };
    
    // Get memory regions
    let regions = memory::get_memory_regions(process)?;
    
    let mut results = Vec::new();
    
    // Scan each region
    for region in regions {
        // Read the memory region
        let data = match memory::read_memory(process, region.start, region.size) {
            Ok(data) => data,
            Err(_) => {
                // Skip regions we can't read
                continue;
            }
        };
        
        // Search for the value in the data
        for i in 0..data.len().saturating_sub(4) {
            if i + 3 < data.len() {
                let bytes: [u8; 4] = [
                    data[i],
                    data[i + 1],
                    data[i + 2],
                    data[i + 3],
                ];
                
                let found_value = u32::from_le_bytes(bytes);
                
                if found_value == parsed_value {
                    results.push(ScanResult {
                        address: region.start + i,
                        value: found_value.to_string(),
                    });
                }
            }
        }
    }
    
    Ok(results)
}

pub fn scan_fuzzy(process: &Process, value: &str, tolerance: f32) -> Result<Vec<ScanResult>> {
    // Parse the value as float
    let target_value: f32 = value.parse()?;
    
    // Get memory regions
    let regions = memory::get_memory_regions(process)?;
    
    let mut results = Vec::new();
    
    // Scan each region
    for region in regions {
        // Read the memory region
        let data = match memory::read_memory(process, region.start, region.size) {
            Ok(data) => data,
            Err(_) => {
                // Skip regions we can't read
                continue;
            }
        };
        
        // Search for the value in the data
        for i in 0..data.len().saturating_sub(4) {
            if i + 3 < data.len() {
                let bytes: [u8; 4] = [
                    data[i],
                    data[i + 1],
                    data[i + 2],
                    data[i + 3],
                ];
                
                let found_value = f32::from_le_bytes(bytes);
                
                if (found_value - target_value).abs() <= tolerance {
                    results.push(ScanResult {
                        address: region.start + i,
                        value: found_value.to_string(),
                    });
                }
            }
        }
    }
    
    Ok(results)
}