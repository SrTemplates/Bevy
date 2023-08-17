# Template for Bevy
This template integrates automated generation using charge-generate to make the initialization of a project much simpler and less tedious, ideal for anyone who wants to speed up their time.

# Template Features
- It includes a feature to separate development mode from release mode (`inspect`)
    > [!NOTE]
    > with this feature the development dependencies are automatically enabled as well as an entity inspector, it is planned to integrate a more complete editor later on.
-  Support Platforms
    - Native
        - MacOs
        - Linux
        - Windows
    - Web (Wasm)
    - Library (Usable in other rust proyects)
    - Mobile
      - Android
      - iOS (:warning: Soon)
- CI workflow for push and pull requests
- CD workflow for all platforms enabled
- Easy deployment of a new version with `cargo release`
- Automate Deploy for these stores
    - Github Release
    - GithubIO (only for web)
    - Itch.io
    - Google Play Store
    - Steam (:warning: Planned)
- Separate game logic and launchers
- Use of `cargo-make` to provide preset tasks which you can see [below](#user-content-other-cargomake-tasks)
    - Just run `cargo make dev` to compile and release a development mode version of the game
        > [!NOTE]
        > It only works if I activate the `pc`, `web` or `all` platform.

# Requirements
- Rust
- Cargo
- [Cargo Generate](https://github.com/cargo-generate/cargo-generate)
- [Cargo Make](https://github.com/sagiegurari/cargo-make) (Optional)
- [Cargo Release](https://github.com/crate-ci/cargo-release) (Optional)
- [Trunk](https://trunkrs.dev) (Optional for web development)

# Use this Template
> [!NOTE]
> If you want support android:
> You need `--allow-commands` to create foders and package
> It is only mandatory if you do not want to be asked for each command before it is executed, although if you prefer you can leave it unchecked so that when it is executed you can see which command is running and decide whether to accept its execution or not.

```sh
cargo generate SrTemplates/Bevy

# Enabling commands to support android
cargo generate -a SrTemplates/Bevy
```
> [!WARNING]
> After initializing this project and having activated the github workflows, you need to configure the secret variables in your github project (This is done this way to protect the security of your data).
Here is the list of each variable required for workflows, note that if you have not activated the functionality, you do not need to configure it
- for Android:
    - KEYSTORE_PASSWORD
        - For this key I recommend to generate a random one (and optionally keep it in a safe place just in case, although theoretically it would not be necessary since you have this autodeployment).
        - Save it temporarily for use in the compilation key generation (KEYSTORE)
    - KEYSTORE
        - For this you have two options [Video Help](https://www.youtube.com/watch?v=ipS7SbyR5Yw&ab_channel=FlutterCoding)
            - [Using Android Studio](https://developer.android.com/studio/publish/app-signing?hl=es-419#generate-key)
            - Using Command Line
                - `keytool -genkey -v -keystore <key_name>.jks -keyalg RSA -keysize 2048 -validity 36525 -alias <key_alias>`
                - Fill in your data and you are ready to go
        - Now to put this key in the github secret variable, just run this command `cat <key_name.jks> | base64`
        - The output of the above command is what you should put in this secret
            > [!WARNING]
            > It is known that this is not the most secure way to store the key and although the secrets cannot be seen once created, I will work to implement a more secure way.
- for Google Play Store:
    - SERVICE_ACCOUNT_JSON
        - You can see more details on how to generate this file [here](https://stackoverflow.com/a/69941050)
        - Then just copy the content in plain text and paste it into the content of this secret
- for itch.io:
    - BUTLER_CREDENTIALS
        - You can obtain it from your [API key](https://itch.io/user/settings/api-keys) user configuration page.
        - Then just copy the content in plain text and paste it into the content of this secret

# Development Guide
- Edit the `.env` file if you need
- Edit `src` folder
- Run `cargo make dev` for run as development mode
- Run `cargo make --list-all-steps` for check all aviable tasks
- To upload a new version and trigger all the workflows related to the deployment of a new version, you just have to run the command `cargo release -x patch` (See the `cargo release -h` for more information)

## Other CargoMake Tasks

* **check** - Check all issues, format and code quality
* **clean** - Clean all target directory
* **clippy** - Check code quality
* **default** - Check all issues, format and code quality
* **dev** - Run native launcher with development configuration
* **fix-all** - Try fix all clippy and format issues
* **fix-fmt** - Fix format
* **fmt** - Check format quality
* **test** - Check all unit test

## :bulb: Tips & tricks
If the template is used on a regular basis, [cargo-generate] allows to setup favorite templates and default variables.

To do this, open or create the file `$CARGO_HOME/cargo-generate.toml`, insert this:
```toml
[values]
deploy_itch = true # or false
deploy_github_pages = true # or false
deploy_play_store = false # or true
gh_username = "<Your GitHub username>"
itch_username = "<Your itch.io username>"
license = "all" # or "MIT" or "APACHE"

[favorites.bevy]
git = "https://github.com/SrTemplates/Bevy"
```

After this, the template can be expanded using `cargo generate bevy`.
