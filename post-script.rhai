let platform = variable::get("platform");

if platform == "mobile" || platform == "all" {
    let pkg = variable::get("pkg_name");

    pkg.replace(".", "/");

    system::command("mkdir", ["-p", `launchers/mobile/android/src/${pkg}`]);
    system::command("mv", [
        "launchers/mobile/android/src/MainActivity.kt",
        `launchers/mobile/android/src/${pkg}`
    ]);
}

variable::set("platform", platform);
