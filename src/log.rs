use sysinfo::{ProcessRefreshKind, ProcessesToUpdate, System};
use chrono::prelude::Local;
use std::{fs::File, io::{Result, Write}, thread, time::Duration};

pub fn write_log(destination_dir: &str) -> Result<File> {
	let file = File::options()
				.create(true)
				.append(true)
				.open(destination_dir)?;

	{
		let mut file = file.try_clone()?;

		thread::spawn(move || -> Result<()> {

			let mut sys = System::new();

			// cpu usage must be computed at least one time outside the loop
			// to have a starting reference cpu usage
			// (otherwise the first calculation always gives 100% utilization)
			sys.refresh_cpu_usage();

			let number_of_cpus: f32 = sys.cpus().len().to_string().parse().unwrap();
			let pid_current_process = sysinfo::get_current_pid().expect("Cannot read PID of the current process!");

			loop{

				let two_minutes = Duration::from_secs(2);
				thread::sleep(two_minutes);

				sys.refresh_processes_specifics(
					ProcessesToUpdate::Some(&[pid_current_process]), 
					ProcessRefreshKind::everything()
				);

				let current_process = sys.process(pid_current_process).unwrap();
				let date_time = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
				let cpu_usage = current_process.cpu_usage() / number_of_cpus;
				write!(file, "[{}] CPU usage: {}%\n", date_time, cpu_usage)?;
			}

		});

	}

	Ok(file)
}
