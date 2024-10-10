use core::time;
use std::{fs::File, io::Write, thread::sleep};
use sysinfo::System;

const WAIT_TIME: time::Duration = time::Duration::from_secs(10);

fn main() {
  let thread = std::thread::spawn(move || {
    log_processes_list();
  });
  let _result = thread.join();
}

fn log_processes_list() {
  let mut system = sysinfo::System::new_all();

  loop {
    // Refreshes all system and processes information.
    system.refresh_all();

    let process_list = get_process_list(&system);
    rewrite_log_list(process_list);

    sleep(WAIT_TIME);
  }
}

fn rewrite_log_list(process_list: String) {
  let mut file = File::create("log.txt").unwrap();
  file.write_all(process_list.as_bytes()).unwrap();
}

fn get_process_list(system: &System) -> String {
  let mut result = String::new();

  for (pid, proc) in system.processes() {
    if proc.exe().is_some() {
      result += &format_process_info(&pid, &proc);
    }
  }

  result
}

fn format_process_info(pid: &sysinfo::Pid, proc: &sysinfo::Process) -> String {
  format!("{:?}[{}]\n", proc.exe().unwrap(), pid)
}
