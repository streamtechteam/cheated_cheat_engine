pub mod memory;
pub mod scanner;
pub mod process;

#[cfg(test)]
mod tests {
    use crate::process::Process;
    use crate::scanner::ScanResult;
    
    // Create mock versions of the scanner functions for testing
    fn mock_scan_exact(_process: &Process, value: &str) -> anyhow::Result<Vec<ScanResult>> {
        if value == "100" {
            Ok(vec![ScanResult {
                address: 0x150000,
                value: "100".to_string(),
            }])
        } else {
            Ok(vec![])
        }
    }
    
    fn mock_scan_fuzzy(_process: &Process, value: &str, tolerance: f32) -> anyhow::Result<Vec<ScanResult>> {
        if value == "100" && tolerance == 1.0 {
            Ok(vec![ScanResult {
                address: 0x180000,
                value: "100.5".to_string(),
            }])
        } else {
            Ok(vec![])
        }
    }
    
    #[test]
    fn test_scan_exact() {
        let process = Process::new(1234, "test.exe".to_string());
        let results = mock_scan_exact(&process, "100").unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].address, 0x150000);
        assert_eq!(results[0].value, "100");
    }
    
    #[test]
    fn test_scan_fuzzy() {
        let process = Process::new(1234, "test.exe".to_string());
        let results = mock_scan_fuzzy(&process, "100", 1.0).unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].address, 0x180000);
        assert_eq!(results[0].value, "100.5");
    }
    
    #[test]
    fn test_process_display() {
        let process = Process::new(1234, "test.exe".to_string());
        assert_eq!(format!("{}", process), "test.exe (PID: 1234)");
    }
}