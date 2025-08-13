//! Memory manipulation functions for the cheat engine
use anyhow::Result;
use crate::process::Process;

/// Represents a region of memory in a process
pub struct MemoryRegion {
    /// The starting address of the memory region
    pub start: usize,
    /// The size of the memory region in bytes
    pub size: usize,
}

#[cfg(windows)]
pub fn read_memory(process: &Process, address: usize, size: usize) -> Result<Vec<u8>> {
    use winapi::um::memoryapi::ReadProcessMemory;
    use winapi::um::winnt::MEMORY_BASIC_INFORMATION;
    use winapi::um::sysinfoapi::GetSystemInfo;
    use winapi::um::winnt::SYSTEM_INFO;
    
    let mut data = vec![0u8; size];
    let mut bytes_read = 0;
    
    unsafe {
        let success = ReadProcessMemory(
            process.handle,
            address as *const winapi::ctypes::c_void,
            data.as_mut_ptr() as *mut winapi::ctypes::c_void,
            size,
            &mut bytes_read,
        );
        
        if success == 0 {
            return Err(anyhow::anyhow!("Failed to read memory"));
        }
        
        // Resize the vector to the actual number of bytes read
        data.resize(bytes_read, 0);
    }
    
    Ok(data)
}

#[cfg(unix)]
pub fn read_memory(process: &Process, address: usize, size: usize) -> Result<Vec<u8>> {
    use libc::{c_void, iovec, process_vm_readv};
    
    let mut data = vec![0u8; size];
    let local_iov = iovec {
        iov_base: data.as_mut_ptr() as *mut c_void,
        iov_len: size,
    };
    
    let remote_iov = iovec {
        iov_base: address as *mut c_void,
        iov_len: size,
    };
    
    let result = unsafe {
        process_vm_readv(
            process.pid as i32,
            &local_iov,
            1,
            &remote_iov,
            1,
            0,
        )
    };
    
    if result == -1 {
        return Err(anyhow::anyhow!("Failed to read memory"));
    }
    
    Ok(data)
}

#[cfg(not(any(windows, unix)))]
pub fn read_memory(_process: &Process, _address: usize, size: usize) -> Result<Vec<u8>> {
    // For demonstration purposes, we'll create mock data that contains some specific values
    // This will allow our scanner to find matches
    let mut data = vec![0; size];
    
    // Insert some mock values that our scanner can find
    // We'll place the value 100 at a specific location
    if _address <= 0x150000 && _address + size > 0x150000 {
        let offset = 0x150000 - _address;
        if offset + 4 <= size {
            // Place the value 100 (0x64) as a little-endian u32
            data[offset] = 0x64;     // 100 in decimal
            data[offset + 1] = 0x00;
            data[offset + 2] = 0x00;
            data[offset + 3] = 0x00;
        }
    }
    
    // Insert the value 200 at another location
    if _address <= 0x250000 && _address + size > 0x250000 {
        let offset = 0x250000 - _address;
        if offset + 4 <= size {
            // Place the value 200 (0xC8) as a little-endian u32
            data[offset] = 0xC8;     // 200 in decimal
            data[offset + 1] = 0x00;
            data[offset + 2] = 0x00;
            data[offset + 3] = 0x00;
        }
    }
    
    // Insert a float value (100.5) for fuzzy matching
    if _address <= 0x180000 && _address + size > 0x180000 {
        let offset = 0x180000 - _address;
        if offset + 4 <= size {
            // Place the float value 100.5 as little-endian f32
            let float_bytes = 100.5f32.to_le_bytes();
            data[offset] = float_bytes[0];
            data[offset + 1] = float_bytes[1];
            data[offset + 2] = float_bytes[2];
            data[offset + 3] = float_bytes[3];
        }
    }
    
    Ok(data)
}

#[cfg(windows)]
pub fn write_memory(process: &Process, address: usize, data: &[u8]) -> Result<()> {
    use winapi::um::memoryapi::WriteProcessMemory;
    
    let mut bytes_written = 0;
    
    unsafe {
        let success = WriteProcessMemory(
            process.handle,
            address as *mut winapi::ctypes::c_void,
            data.as_ptr() as *const winapi::ctypes::c_void,
            data.len(),
            &mut bytes_written,
        );
        
        if success == 0 || bytes_written != data.len() {
            return Err(anyhow::anyhow!("Failed to write memory"));
        }
    }
    
    Ok(())
}

#[cfg(unix)]
pub fn write_memory(process: &Process, address: usize, data: &[u8]) -> Result<()> {
    use libc::{c_void, iovec, process_vm_writev};
    
    let local_iov = iovec {
        iov_base: data.as_ptr() as *mut c_void,
        iov_len: data.len(),
    };
    
    let remote_iov = iovec {
        iov_base: address as *mut c_void,
        iov_len: data.len(),
    };
    
    let result = unsafe {
        process_vm_writev(
            process.pid as i32,
            &local_iov,
            1,
            &remote_iov,
            1,
            0,
        )
    };
    
    if result == -1 || result as usize != data.len() {
        return Err(anyhow::anyhow!("Failed to write memory"));
    }
    
    Ok(())
}

#[cfg(not(any(windows, unix)))]
pub fn write_memory(_process: &Process, _address: usize, _data: &[u8]) -> Result<()> {
    // For now, we'll just return Ok
    Ok(())
}

#[cfg(windows)]
pub fn get_memory_regions(process: &Process) -> Result<Vec<MemoryRegion>> {
    use winapi::um::memoryapi::VirtualQueryEx;
    use winapi::um::winnt::{MEMORY_BASIC_INFORMATION, PAGE_NOACCESS};
    
    let mut regions = Vec::new();
    let mut address = 0;
    
    loop {
        let mut mem_info: MEMORY_BASIC_INFORMATION = unsafe { std::mem::zeroed() };
        
        let result = unsafe {
            VirtualQueryEx(
                process.handle,
                address as *const winapi::ctypes::c_void,
                &mut mem_info,
                std::mem::size_of::<MEMORY_BASIC_INFORMATION>(),
            )
        };
        
        if result == 0 {
            break;
        }
        
        // Check if this is a valid region we can read
        if mem_info.State & winapi::um::winnt::MEM_COMMIT != 0 &&
           mem_info.Protect != PAGE_NOACCESS &&
           mem_info.RegionSize > 0 {
            regions.push(MemoryRegion {
                start: mem_info.BaseAddress as usize,
                size: mem_info.RegionSize,
            });
        }
        
        // Move to the next region
        address = mem_info.BaseAddress as usize + mem_info.RegionSize;
        
        // Prevent infinite loop
        if address == 0 {
            break;
        }
    }
    
    Ok(regions)
}

#[cfg(unix)]
pub fn get_memory_regions(process: &Process) -> Result<Vec<MemoryRegion>> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    
    let mut regions = Vec::new();
    
    // Read memory maps from /proc/[pid]/maps
    let maps_path = format!("/proc/{}/maps", process.id);
    let maps_file = File::open(&maps_path)?;
    let reader = BufReader::new(maps_file);
    
    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split_whitespace().collect();
        
        if parts.len() >= 2 {
            // Parse the address range
            let addr_parts: Vec<&str> = parts[0].split('-').collect();
            if addr_parts.len() == 2 {
                if let (Ok(start), Ok(end)) = (
                    usize::from_str_radix(addr_parts[0], 16),
                    usize::from_str_radix(addr_parts[1], 16),
                ) {
                    let size = end - start;
                    
                    // Only include readable regions
                    if parts[1].contains('r') && size > 0 {
                        regions.push(MemoryRegion {
                            start,
                            size,
                        });
                    }
                }
            }
        }
    }
    
    Ok(regions)
}

#[cfg(not(any(windows, unix)))]
pub fn get_memory_regions(_process: &Process) -> Result<Vec<MemoryRegion>> {
    // For now, we'll return some mock regions
    Ok(vec![
        MemoryRegion { start: 0x100000, size: 0x100000 },
        MemoryRegion { start: 0x200000, size: 0x100000 },
    ])
}