use monitor_central::Stat;
use sysinfo::{System, SystemExt};

fn main() {
    let mut sys = System::new_all();
    sys.refresh_all();

    let stat_data = Stat {
        system_identifier: sys.host_name().unwrap_or("unknown".to_string()),
        total_memory: sys.total_memory(),
        used_memory: sys.used_memory(),
        total_swap: sys.total_swap(),
        used_swap: sys.used_swap(),
        uptime: sys.uptime(),
    };

    println!("{:?}", stat_data);
}
