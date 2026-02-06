use sysinfo::{System, Disks as SysDisks, Pid, ProcessesToUpdate};

struct SystemMonitor {
    sys: System,
}

struct DiskInfo {
    name: String,
    total: u64,
    used: u64,
}

struct ProcessInfo {
    pid: u32,
    name: String,
    cpu_usage: f32,
}

impl SystemMonitor {
    fn new() -> Self {
        let mut sys = System::new_all();
        sys.refresh_all();
        Self { sys }
    }

    fn update(&mut self) {
        self.sys.refresh_cpu_all();
        self.sys.refresh_memory();
    }

    pub fn collect_cpu(&self) -> f32 {
        self.sys.global_cpu_usage()
    }

    pub fn collect_memory(&self) -> (u64, u64) {
        let used = self.sys.used_memory();
        let total = self.sys.total_memory();
        
        let used_mb = used / 1_024 / 1_024;
        let total_mb = total / 1_024 / 1_024;
        
        (used_mb, total_mb)
    }

    pub fn collect_disks(&mut self) -> Vec<DiskInfo> {
        let disks = SysDisks::new_with_refreshed_list();
        let mut disks_info = Vec::<DiskInfo>::new();
        
        for disk in disks.list() {
            let total = disk.total_space();
            let available = disk.available_space();
            let used = total - available;
            disks_info.push(DiskInfo {
                name: disk.name().to_string_lossy().to_string(),
                total,
                used,
            });
        }
        disks_info
    }

    pub fn collect_processes(&mut self) -> (Vec<ProcessInfo>, usize) {
        self.sys.refresh_processes(ProcessesToUpdate::All);
        let processes = self.sys.processes();
        let mut processes_info = Vec::<ProcessInfo>::new();
        
        for (pid, process) in processes.iter() {
            processes_info.push(ProcessInfo {
                pid: pid.as_u32(),
                name: process.name().to_string_lossy().to_string(),
                cpu_usage: process.cpu_usage(),
            });
        }
        (processes_info, processes.len())
    }

    pub fn display(&mut self) {
        print!("\x1B[2J\x1B[1;1H");
        
        println!("========== SYSTEM MONITOR ==========\n");

        let cpu_usage = self.collect_cpu();
        println!("CPU Usage: {}{:.2}%\x1B[0m", Self::get_color(cpu_usage), cpu_usage);

        let (used_mem, total_mem) = self.collect_memory();
        let mem_percentage = (used_mem as f32 / total_mem as f32) * 100.0;
        println!("Memory: {}{} MB\x1B[0m / {} MB ({:.1}%)", 
            Self::get_color(mem_percentage), used_mem, total_mem, mem_percentage);

        println!("\nDisks:");
        let disks = self.collect_disks();
        for disk in &disks {
            let disk_percentage = (disk.used as f32 / disk.total as f32) * 100.0;
            let (used_display, used_unit) = Self::format_bytes(disk.used);
            let (total_display, total_unit) = Self::format_bytes(disk.total);
            
            println!("  {}: {}{:.2} {}\x1B[0m / {:.2} {} ({:.1}%)", 
                disk.name,
                Self::get_color(disk_percentage), 
                used_display, used_unit, 
                total_display, total_unit, 
                disk_percentage);
        }

        println!("\nTop 10 Processes:");
        let (mut processes, _total_count) = self.collect_processes();
        processes.sort_by(|a, b| b.cpu_usage.partial_cmp(&a.cpu_usage).unwrap());
        
        for process in processes.iter().take(10) {
            println!("  PID: {:>6} | CPU: {}{:>6.2}%\x1B[0m | {}", 
                process.pid,
                Self::get_color(process.cpu_usage),
                process.cpu_usage,
                process.name
            );
        }
        
        println!("\n====================================");
    }

    fn get_color(percentage: f32) -> &'static str {
        if percentage >= 80.0 {
            "\x1B[91m"
        } else if percentage >= 60.0 {
            "\x1B[93m"
        } else {
            "\x1B[92m"
        }
    }

    fn format_bytes(bytes: u64) -> (f64, &'static str) {
        const GB: u64 = 1_073_741_824;
        const MB: u64 = 1_048_576;
        const KB: u64 = 1_024;

        if bytes >= GB {
            ((bytes as f64) / (GB as f64), "GB")
        } else if bytes >= MB {
            ((bytes as f64) / (MB as f64), "MB")
        } else if bytes >= KB {
            ((bytes as f64) / (KB as f64), "KB")
        } else {
            (bytes as f64, "B")
        }
    }
}

fn main() {
    let mut monitor = SystemMonitor::new();
    
    monitor.update();
    monitor.display();
        
}