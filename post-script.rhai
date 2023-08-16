let platform = variable::get("platform");

if platform == "mobile" || platform == "all" {
    // let pkg = variable::prompt("What will be the name of the package to be used in the android project? (ex: com.sergioribera.supergame)");
    // variable::set("pkg_name", pkg);
    let pkg = variable::get("pkg_name");

    pkg.replace(".", "/");
    variable::set("pkg_folder", pkg);
}

variable::set("platform", platform);

if platform != "mobile" && platform != "all" {
    print("Removing Mobile");
    file::delete(".github/workflows/build-android.yml");
    file::delete("launchers/mobile");
    file::delete("res");
}

if platform != "web" && platform != "all" {
    print("Removing Web");
    file::delete(".github/workflows/release-gh-pages.yml");
    file::delete(".github/workflows/release-itch-html.yml");
    file::delete("launchers/wasm");
}
