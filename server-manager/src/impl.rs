use super::*;

/// Provides a default implementation for `ServerManager`.
impl Default for ServerManager {
    /// Creates a default `ServerManager` instance with empty hooks and no PID file configured.
    #[inline(always)]
    fn default() -> Self {
        let empty_hook: ServerManagerHook = Arc::new(|| Box::pin(async {}));
        Self {
            pid_file: Default::default(),
            stop_hook: empty_hook.clone(),
            server_hook: empty_hook.clone(),
            start_hook: empty_hook,
        }
    }
}

/// Implementation of server management operations.
///
/// Provides methods for starting, stopping and managing server processes.
impl ServerManager {
    /// Creates a new `ServerManager` instance.
    ///
    /// This is a convenience method that calls `ServerManager::default()`.
    #[inline(always)]
    pub fn new() -> Self {
        Self::default()
    }

    /// Starts the server in foreground mode.
    ///
    /// Writes the current process ID to the PID file and executes the server function.
    pub async fn start(&self) {
        (self.get_start_hook())().await;
        if let Err(e) = self.write_pid_file() {
            eprintln!("Failed to write pid file: {e}");
            return;
        }
        (self.get_server_hook())().await;
    }

    /// Stops the running server process.
    ///
    /// Reads PID from file and terminates the process.
    ///
    /// # Returns
    ///
    /// - `ServerManagerResult` - Operation result.
    pub async fn stop(&self) -> ServerManagerResult {
        (self.get_stop_hook())().await;
        let pid: i32 = self.read_pid_file()?;
        self.kill_process(pid)
    }

    /// Starts the server in daemon (background) mode on Unix platforms.
    ///
    /// # Returns
    ///
    /// - `ServerManagerResult` - Operation result.
    #[cfg(not(windows))]
    pub async fn start_daemon(&self) -> ServerManagerResult {
        (self.get_start_hook())().await;
        if std::env::var(RUNNING_AS_DAEMON).is_ok() {
            self.write_pid_file()?;
            let rt: Runtime = Runtime::new()?;
            rt.block_on(async {
                (self.get_server_hook())().await;
            });
            return Ok(());
        }
        let exe_path: PathBuf = std::env::current_exe()?;
        let mut cmd: Command = Command::new(exe_path);
        cmd.env(RUNNING_AS_DAEMON, RUNNING_AS_DAEMON_VALUE)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .stdin(Stdio::null());
        cmd.spawn()
            .map_err(|error: Error| Box::new(error) as Box<dyn std::error::Error>)?;
        Ok(())
    }

    /// Starts the server in daemon (background) mode on Windows platforms.
    ///
    /// # Returns
    ///
    /// - `ServerManagerResult` - Operation result.
    #[cfg(windows)]
    pub async fn start_daemon(&self) -> ServerManagerResult {
        (self.get_start_hook())().await;
        if std::env::var(RUNNING_AS_DAEMON).is_ok() {
            self.write_pid_file()?;
            let rt: Runtime = Runtime::new()?;
            rt.block_on(async {
                (self.get_server_hook())().await;
            });
            return Ok(());
        }
        let exe_path: PathBuf = std::env::current_exe()?;
        let mut cmd: Command = Command::new(exe_path);
        cmd.env(RUNNING_AS_DAEMON, RUNNING_AS_DAEMON_VALUE)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .stdin(Stdio::null())
            .creation_flags(0x00000008);
        cmd.spawn()
            .map_err(|error: Error| Box::new(error) as Box<dyn std::error::Error>)?;
        Ok(())
    }

    /// Reads process ID from the PID file.
    ///
    /// # Returns
    ///
    /// - `Result<i32, Box<dyn std::error::Error>>` - Process ID if successful.
    fn read_pid_file(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let pid_str: String = fs::read_to_string(self.get_pid_file())?;
        let pid: i32 = pid_str.trim().parse::<i32>()?;
        Ok(pid)
    }

    /// Writes current process ID to the PID file.
    ///
    /// # Returns
    ///
    /// - `ServerManagerResult` - Operation result.
    fn write_pid_file(&self) -> ServerManagerResult {
        if let Some(parent) = Path::new(self.get_pid_file()).parent() {
            fs::create_dir_all(parent)?;
        }
        let pid: u32 = id();
        fs::write(self.get_pid_file(), pid.to_string())?;
        Ok(())
    }

    /// Kills process by PID on Unix platforms.
    ///
    /// # Arguments
    ///
    /// - `i32` - The ID of the process to terminate.
    ///
    /// # Returns
    ///
    /// - `ServerManagerResult` - Operation result.
    #[cfg(not(windows))]
    fn kill_process(&self, pid: i32) -> ServerManagerResult {
        match Command::new(KILL)
            .arg(KILL_SIGNAL)
            .arg(pid.to_string())
            .output()
        {
            Ok(output) if output.status.success() => Ok(()),
            Ok(output) => Err(format!(
                "Failed to kill process with pid: {}, error: {}",
                pid,
                String::from_utf8_lossy(&output.stderr)
            )
            .into()),
            Err(e) => Err(format!("Failed to execute kill command: {}", e).into()),
        }
    }

    /// Kills process by PID on Windows platforms.
    ///
    /// # Arguments
    ///
    /// - `i32` - The ID of the process to terminate.
    ///
    /// # Returns
    ///
    /// - `ServerManagerResult` - Operation result.
    #[cfg(windows)]
    fn kill_process(&self, pid: i32) -> ServerManagerResult {
        unsafe extern "system" {
            /// Opens a process object with the requested access rights.
            ///
            /// # Arguments
            ///
            /// - `u32` - The access rights requested for the process handle.
            /// - `i32` - Whether the returned handle is inheritable by child processes.
            /// - `u32` - The identifier of the process to open.
            ///
            /// # Returns
            ///
            /// - `*mut c_void` - The process handle, or a null pointer on failure.
            fn OpenProcess(
                dwDesiredAccess: u32,
                bInheritHandle: i32,
                dwProcessId: u32,
            ) -> *mut c_void;
            /// Terminates a process and all of its child processes.
            ///
            /// # Arguments
            ///
            /// - `*mut c_void` - The process handle returned by `OpenProcess`.
            /// - `u32` - The exit code reported for the terminated process.
            ///
            /// # Returns
            ///
            /// - `i32` - Non-zero when the process was terminated, zero on failure.
            fn TerminateProcess(hProcess: *mut c_void, uExitCode: u32) -> i32;
            /// Closes an open process handle.
            ///
            /// # Arguments
            ///
            /// - `*mut c_void` - The process handle to close.
            ///
            /// # Returns
            ///
            /// - `i32` - Non-zero when the handle was closed, zero on failure.
            fn CloseHandle(hObject: *mut c_void) -> i32;
            /// Reads the calling thread's last recorded Win32 error code.
            ///
            /// # Returns
            ///
            /// - `u32` - The last recorded error code.
            fn GetLastError() -> u32;
        }
        let process_id: u32 = pid as u32;
        let mut process_handle: *mut c_void = unsafe { OpenProcess(0x0001, 0, process_id) };
        if process_handle.is_null() {
            process_handle = unsafe { OpenProcess(0x1F0FFF, 0, process_id) };
        }
        if process_handle.is_null() {
            let error_code: u32 = unsafe { GetLastError() };
            return Err(format!(
                "Failed to open process with pid: {pid}. Error code: {error_code}"
            )
            .into());
        }
        let terminate_result: i32 = unsafe { TerminateProcess(process_handle, 1) };
        if terminate_result == 0 {
            let error_code: u32 = unsafe { GetLastError() };
            unsafe {
                CloseHandle(process_handle);
            }
            return Err(format!(
                "Failed to terminate process with pid: {pid}. Error code: {error_code}"
            )
            .into());
        }
        unsafe {
            CloseHandle(process_handle);
        }
        Ok(())
    }

    /// Runs the server with cargo-watch.
    ///
    /// # Arguments
    ///
    /// - `&[&str]` - A slice of string arguments to pass to `cargo-watch`.
    /// - `bool` - A boolean indicating whether to wait for the `cargo-watch` process to complete.
    ///
    /// # Returns
    ///
    /// - `ServerManagerResult` - Operation result.
    async fn run_with_cargo_watch(&self, run_args: &[&str], wait: bool) -> ServerManagerResult {
        (self.get_start_hook())().await;
        let cargo_watch_installed: Output = Command::new(CARGO)
            .arg(INSTALL)
            .args(CARGO_WATCH_INSTALL_LIST_ARGS)
            .output()?;
        if !String::from_utf8_lossy(&cargo_watch_installed.stdout).contains(CARGO_WATCH) {
            eprintln!("{CARGO_WATCH_ABSENT_MESSAGE}");
            let install_status: ExitStatus = Command::new(CARGO)
                .args(CARGO_INSTALL_ARGS)
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .spawn()?
                .wait()?;
            if !install_status.success() {
                return Err(CARGO_WATCH_INSTALL_FAILED_MESSAGE.into());
            }
            eprintln!("{CARGO_WATCH_INSTALLED_MESSAGE}");
        }
        let mut command: Command = Command::new(CARGO_WATCH);
        command
            .args(run_args)
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .stdin(Stdio::inherit());
        let mut child: Child = command
            .spawn()
            .map_err(|error: Error| Box::new(error) as Box<dyn std::error::Error>)?;
        if wait {
            child
                .wait()
                .map_err(|error: Error| Box::new(error) as Box<dyn std::error::Error>)?;
        }
        exit(0);
    }

    /// Starts the server with hot-reloading using `cargo-watch` in detached mode.
    ///
    /// This function spawns `cargo-watch` and returns immediately.
    ///
    /// # Arguments
    ///
    /// - `&[&str]` - A slice of string arguments to pass to `cargo-watch`.
    ///
    /// # Returns
    ///
    /// - `ServerManagerResult` - Operation result.
    pub async fn watch_detached(&self, run_args: &[&str]) -> ServerManagerResult {
        self.run_with_cargo_watch(run_args, false).await
    }

    /// Starts the server with hot-reloading using `cargo-watch` and waits for it to complete.
    ///
    /// This function is blocking and will wait for the `cargo-watch` process to exit.
    ///
    /// # Arguments
    ///
    /// - `&[&str]` - A slice of string arguments to pass to `cargo-watch`.
    ///
    /// # Returns
    ///
    /// - `ServerManagerResult` - Operation result.
    pub async fn watch(&self, run_args: &[&str]) -> ServerManagerResult {
        self.run_with_cargo_watch(run_args, true).await
    }
}
