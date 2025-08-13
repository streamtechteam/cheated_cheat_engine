use anyhow::Result;
use std::fmt;

#[derive(Debug, Clone)]
pub struct Process {
    pub id: u32,
    pub name: String,
    #[cfg(windows)]
    pub handle: winapi::um::winnt::HANDLE,
    #[cfg(unix)]
    pub pid: libc::pid_t,
}

impl Process {
    pub fn new(id: u32, name: String) -> Self {
        Self {
            id,
            name,
            #[cfg(windows)]
            handle: std::ptr::null_mut(),
            #[cfg(unix)]
            pid: id as libc::pid_t,
        }
    }
}

impl fmt::Display for Process {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} (PID: {})", self.name, self.id)
    }
}

#[cfg(windows)]
pub fn attach(process_name: &str) -> Result<Process> {
    use winapi::um::tlhelp32::*;
    use winapi::um::handleapi::*;
    use winapi::um::processthreadsapi::*;
    use std::ffi::CStr;
    
    unsafe {
        // Create a snapshot of all processes
        let snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0);
        if snapshot == INVALID_HANDLE_VALUE {
            return Err(anyhow::anyhow!("Failed to create process snapshot"));
        }
        
        // Initialize process entry structure
        let mut entry: PROCESSENTRY32 = std::mem::zeroed();
        entry.dwSize = std::mem::size_of::<PROCESSENTRY32>() as u32;
        
        // Get the first process
        if Process32First(snapshot, &mut entry) != 0 {
            loop {
                // Convert the process name from C string
                let name = CStr::from_ptr(entry.szExeFile.as_ptr())
                    .to_string_lossy()
                    .into_owned();
                
                // Check if this is the process we're looking for
                if name == process_name {
                    // Open a handle to the process
                    let handle = OpenProcess(
                        winapi::um::winnt::PROCESS_VM_READ | 
                        winapi::um::winnt::PROCESS_VM_WRITE | 
                        winapi::um::winnt::PROCESS_VM_OPERATION,
                        0,
                        entry.th32ProcessID
                    );
                    
                    if handle != std::ptr::null_mut() {
                        // Close the snapshot handle
                        CloseHandle(snapshot);
                        
                        // Return the process with handle
                        return Ok(Process {
                            id: entry.th32ProcessID,
                            name,
                            handle,
                        });
                    }
                }
                
                // Get the next process
                if Process32Next(snapshot, &mut entry) == 0 {
                    break;
                }
            }
        }
        
        // Close the snapshot handle
        CloseHandle(snapshot);
        
        Err(anyhow::anyhow!("Process '{}' not found", process_name))
    }
}

#[cfg(unix)]
pub fn attach(process_name: &str) -> Result<Process> {
    use std::fs;
    use std::path::Path;
    
    // Read the /proc directory
    let proc_dir = Path::new("/proc");
    if !proc_dir.exists() {
        return Err(anyhow::anyhow!("Failed to access /proc directory"));
    }
    
    // Iterate through all entries in /proc
    for entry in fs::read_dir(proc_dir)? {
        let entry = entry?;
        let file_name = entry.file_name();
        
        // Check if the entry is a directory with a numeric name (PID)
        if let Ok(pid) = file_name.to_string_lossy().parse::<u32>() {
            // Try to read the process name from /proc/[pid]/comm
            let comm_path = proc_dir.join(pid.to_string()).join("comm");
            if let Ok(name) = fs::read_to_string(comm_path) {
                let name = name.trim().to_string();
                if !name.is_empty() && name == process_name {
                    // Return the process
                    return Ok(Process::new(pid, name));
                }
            }
        }
    }
    
    Err(anyhow::anyhow!("Process '{}' not found", process_name))
}

#[cfg(not(any(windows, unix)))]
pub fn attach(process_name: &str) -> Result<Process> {
    // Fallback for other platforms
    Err(anyhow::anyhow!("Process attachment not supported on this platform"))
}

#[cfg(windows)]
pub fn list_processes() -> Result<Vec<Process>> {
    use winapi::um::tlhelp32::*;
    use winapi::um::handleapi::*;
    use winapi::um::processthreadsapi::*;
    use std::ffi::CStr;
    
    let mut processes = Vec::new();
    
    unsafe {
        // Create a snapshot of all processes
        let snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0);
        if snapshot == INVALID_HANDLE_VALUE {
            return Err(anyhow::anyhow!("Failed to create process snapshot"));
        }
        
        // Initialize process entry structure
        let mut entry: PROCESSENTRY32 = std::mem::zeroed();
        entry.dwSize = std::mem::size_of::<PROCESSENTRY32>() as u32;
        
        // Get the first process
        if Process32First(snapshot, &mut entry) != 0 {
            loop {
                // Convert the process name from C string
                let name = CStr::from_ptr(entry.szExeFile.as_ptr())
                    .to_string_lossy()
                    .into_owned();
                
                // Add the process to our list
                processes.push(Process::new(entry.th32ProcessID, name));
                
                // Get the next process
                if Process32Next(snapshot, &mut entry) == 0 {
                    break;
                }
            }
        }
        
        // Close the snapshot handle
        CloseHandle(snapshot);
    }
    
    Ok(processes)
}

#[cfg(unix)]
pub fn list_processes() -> Result<Vec<Process>> {
    use std::fs;
    use std::path::Path;
    
    let mut processes = Vec::new();
    
    // Read the /proc directory
    let proc_dir = Path::new("/proc");
    if !proc_dir.exists() {
        return Err(anyhow::anyhow!("Failed to access /proc directory"));
    }
    
    // Iterate through all entries in /proc
    for entry in fs::read_dir(proc_dir)? {
        let entry = entry?;
        let file_name = entry.file_name();
        
        // Check if the entry is a directory with a numeric name (PID)
        if let Ok(pid) = file_name.to_string_lossy().parse::<u32>() {
            // Try to read the process name from /proc/[pid]/comm
            let comm_path = proc_dir.join(pid.to_string()).join("comm");
            if let Ok(name) = fs::read_to_string(comm_path) {
                let name = name.trim().to_string();
                if !name.is_empty() {
                    processes.push(Process::new(pid, name));
                }
            }
        }
    }
    
    Ok(processes)
}

#[cfg(not(any(windows, unix)))]
pub fn list_processes() -> Result<Vec<Process>> {
    // Fallback for other platforms
    let mock_processes = vec![
        Process::new(1234, "game.exe".to_string()),
        Process::new(5678, "browser.exe".to_string()),
        Process::new(9012, "editor.exe".to_string()),
    ];
    
    Ok(mock_processes)
}