# Emergency Backup Project

A project written in Rust for the Programmazione di Sistema course @ Politecnico di Torino.

## Task

Realize an application written in Rust for a PC that let the user to perform a backup whenever his/her display is not working. When the user wants to perform a backup on an external disk, e.g. a USB drive, he/she will have to manually execute a conventional command using the mouse pointer. The command to be chosen is arbitrary, but it could be an X inside the display area or a rectangular following the borders of the display. After the backup initialization command is executed, the user will have to confirm using another command, again using the mouse pointer (it could be a + or a - inside the display area). The backup source is indicated during the setup-up of the tool, e.g. in a configuration file. After the initialization command, the programm will display a confirmation window. The program can also include several kind of backups, e.g. the contents of a folder or all files with a specific extension.

- The application must be installed at bootstrap time and must be active in background.
- The application must use the least CPU consumption possible.
- To assess the CPU consumption it required that a log file is written every 2 minutes with employed CPU consumption.
- Finally, at the end of the backup and in the same external drive, a log file must be created to indicate the total size of the saved files and the CPU time employed to complete the backup operation.

> [!NOTE]  
> Since we strongly believe that using the mouse pointer to activate and confirm the backup could lead to combinations being misinterpreted as program commands, we decided to use _strange_ keyboard bindings to accomplish this task, as better explained in the next section.

## User Manual

### Installation

To install the application, it is sufficient to place the `.exe` file (after the project has been compiled) inside a folder, e.g. in a folder contained in your user directory.

Then you can run it, for example by double-clicking on it, and the application will automatically set up to auto-launch itself when the system starts up. The application will run in background.

The first time you run the application, it will also automatically create a config directory containing a configuration file and a log file. The directory structure will be:

```
📂root_folder
 ┗ 📂config
   ┣ 📜log.txt
   ┗ 📜config.json
```

The `config.json` file is organized as follows:

```json
{
	"config_file":"<config file location>",
	"source":"<source backup directory>",
	"destination":"<destination backup directory>",
	"log_file":"<log file location>"
}
```

Its default content is:

```json
{
	"config_file":"<path to the root folder>/config/config.json",
	"source":"source/",
	"destination":"destination/",
	"log_file":"<path to the root folder>/config/log.txt"
}
```

> [!NOTE]  
> We suggest to create the config directory and the config file manually, in order to set up it before the application is run for the first time. In this manner, the user can also choose the log file location in a more accurate way. It is not mandatory neither to create the log file in advance nor to place it in the config directory.

At the next system start-up, the application will already be running.

### Usage

To start the backup, the initialization command is <kbd>Ctrl Left</kbd> + <kbd>Shift Left</kbd> + <kbd>Alt</kbd> + <kbd>B</kbd>. They must be pressed simultaneously. You need to use <kbd>Option ⌥</kbd> instead of <kbd>Alt</kbd> if you are running on MacOs.

A confimation window will display and a sound will be emitted. You can close it or proceed with the backup by clicking on the `Confirm` button or by pressing <kbd>Ctrl</kbd> + <kbd>B</kbd> at the same time (here <kbd>Ctrl</kbd> can be either left or right). You need to use <kbd>Command ⌘</kbd> instead of <kbd>Ctrl</kbd> if you are running on MacOs.

Every two minutes a new line will be written on the log file with the CPU consumption. The format is the following:

```
[{YYYY/MM/DD HH:MM:SS}] CPU usage: {percentage CPU consumption}%
```

> [!WARNING]  
> For debug reasons, a new line is written every two seconds instead of two minutes. This setting can be modified [here](https://github.com/ProgrammazioneDiSistema2024-IA-ZZ/Group1/blob/8a022a35e23bdb739c07e7c052aa0993d19a6d8e/src/log.rs#L28C1-L28C46).

Upon the backup completion, a new sound will be emitted and a completion text will appear on the confirmation window.

The external drive will contain the backup and a new file named `backup_summary.txt` in the destination folder. This file will be updated every time a backup is executed.
A new line will be written with the following format:

```
[{YYYY/MM/DD HH:MM:SS}] Bytes written: {total number of Bytes} B, CPU time: {CPU time usage}
```

## Design Manual

The project is organized in modules. A file contains a single module.

#### app.rs

This module represents the application core. It handles the windows and the interaction with the user when the backup initialization command is executed. The backup is executed by this module.

#### backup.rs

This module handles the backup. The code contained in the module can perform a backup of an entire directory structure or a single file. It also manages the `backup_summary.txt` file.

#### config.rs

This module handles the configuration file, its creation and its maintenance.

#### lib.rs

This file simply exports the modules and sets their visibility.

#### log.rs

This module handles the write operations on the log file. Basically, it spawns a new thread in charge of periodically writing to the log file.

#### main.rs

This module contains the main function, which sets up the automatic start of the application and starts it. 
It manages the backup initialization command to spawn the application main window.

#### utils.rs

This module simply exports funcctions to emit sounds.

## Authors

- [Paola Pitarresi](https://github.com/pit4r)
- [Davide Sferrazza](https://github.com/FarInHeight)
- [Davide Vitabile](https://github.com/Vitabile)

## License
[MIT License](LICENSE)
