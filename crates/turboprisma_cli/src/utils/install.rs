use std::path::PathBuf;

#[derive(Clone, Copy)]
pub enum PackageManager {
    Npm,
    Yarn,
    Pnpm,
}

pub fn get_package_manager(cwd: &PathBuf) -> PackageManager {
    use lazy_static::lazy_static;
    use std::sync::Mutex;
    lazy_static! {
        static ref LAST_VAL: Mutex<Option<PackageManager>> = Mutex::new(None);
    }
    let mut last = LAST_VAL.lock().unwrap();
    let r = last
        .and_then(|v| Some(v))
        .or_else(|| Some(compute_package_manager(cwd)));
    let v = r.unwrap();
    *last = Some(v);
    v
}

fn compute_package_manager(cwd: &PathBuf) -> PackageManager {
    let result = check_using_lock(cwd);
    match result {
        Some(value) => value,
        None => {
            let result = compute_using_command(cwd);
            match result {
                Some(value) => value,
                None => PackageManager::Npm,
            }
        }
    }
}

fn check_using_lock(cwd: &PathBuf) -> Option<PackageManager> {
    let lock_file = cwd.join("yarn.lock");
    if lock_file.exists() {
        Some(PackageManager::Yarn)
    } else {
        let lock_file = cwd.join("pnpm-lock.yaml");
        if lock_file.exists() {
            Some(PackageManager::Pnpm)
        } else {
            let lock_file = cwd.join("package-lock.json");
            if lock_file.exists() {
                Some(PackageManager::Npm)
            } else {
                None
            }
        }
    }
}

fn compute_using_command(cwd: &PathBuf) -> Option<PackageManager> {
    use std::process::Command;
    let output = Command::new("yarn").arg("-v").output();

    match output {
        Ok(output) => {
            if output.status.success() {
                Some(PackageManager::Yarn)
            } else {
                let output = Command::new("pnpm").arg("-v").output();
                match output {
                    Ok(output) => {
                        if output.status.success() {
                            Some(PackageManager::Pnpm)
                        } else {
                            None
                        }
                    }
                    Err(_) => None,
                }
            }
        }
        Err(_) => {
            let output = Command::new("pnpm").arg("-v").output();
            match output {
                Ok(output) => {
                    if output.status.success() {
                        Some(PackageManager::Pnpm)
                    } else {
                        None
                    }
                }
                Err(_) => None,
            }
        }
    }
}
