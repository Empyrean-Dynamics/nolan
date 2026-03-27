fn main() {
    // Re-run if git state changes
    println!("cargo:rerun-if-changed=.git/HEAD");
    println!("cargo:rerun-if-changed=.git/refs/heads/");
    println!("cargo:rerun-if-changed=.git/refs/tags/");

    let hash = run_git(&["rev-parse", "--short", "HEAD"]).unwrap_or_else(|| "unknown".into());

    let dirty = run_git(&["status", "--porcelain"])
        .map(|s| !s.is_empty())
        .unwrap_or(false);

    // Use git describe to find the latest version tag and distance from it.
    // Format: "v1.0.0" (exactly on tag) or "v1.0.0-3-gabcdef" (3 commits after tag)
    let version = match run_git(&["describe", "--tags", "--match", "v*"]) {
        Some(describe) => {
            if describe.contains('-') {
                // v1.0.0-3-gabcdef → we're past the tag, bump patch and add -dev
                let tag = describe.split('-').next().unwrap_or(&describe);
                let base = tag.strip_prefix('v').unwrap_or(tag);
                let bumped = bump_patch(base);
                format!("{bumped}-dev")
            } else {
                // Exactly on the tag: v1.0.0
                let base = describe.strip_prefix('v').unwrap_or(&describe);
                base.to_string()
            }
        }
        None => {
            // No tags yet, fall back to Cargo.toml version + -dev
            format!("{}-dev", env!("CARGO_PKG_VERSION"))
        }
    };

    // On a tagged release the tag is the identity — only add the hash for dev builds.
    let on_tag = run_git(&["describe", "--tags", "--match", "v*", "--exact-match"]).is_some();

    let suffix = match (on_tag, dirty) {
        (true, false) => String::new(),            // 1.0.0
        (true, true) => "+dirty".into(),           // 1.0.0+dirty
        (false, false) => format!("+{hash}"),      // 1.0.1-dev+f82c1a0
        (false, true) => format!("+{hash}-dirty"), // 1.0.1-dev+f82c1a0-dirty
    };

    println!("cargo:rustc-env=GIT_VERSION={version}{suffix}");
}

fn run_git(args: &[&str]) -> Option<String> {
    std::process::Command::new("git")
        .args(args)
        .output()
        .ok()
        .filter(|o| o.status.success())
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
}

/// Bump the patch component: "1.0.0" → "1.0.1"
fn bump_patch(version: &str) -> String {
    let parts: Vec<&str> = version.split('.').collect();
    if parts.len() == 3
        && let Ok(patch) = parts[2].parse::<u64>()
    {
        return format!("{}.{}.{}", parts[0], parts[1], patch + 1);
    }
    version.to_string()
}
