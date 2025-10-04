use {
    std::process::Command,
    sysinfo::{Pid, System},
};

fn main() {
    let parent_pid = {
        let system = System::new_all();
        system
            .process(Pid::from(std::process::id() as usize))
            .and_then(|p| p.parent())
            .expect("Failed getting the parent pid")
    };

    #[cfg(unix)]
    Command::new("kill")
        .arg("-9")
        .arg(parent_pid.to_string())
        .output()
        .unwrap();

    #[cfg(windows)]
    Command::new("taskkill")
        .args(&["/PID", &parent_pid.to_string(), "/F"])
        .output()
        .unwrap();
}
