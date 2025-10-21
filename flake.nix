{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    nixpkgs,
    flake-utils,
    rust-overlay,
    ...
  }:
    flake-utils.lib.eachSystem (flake-utils.lib.defaultSystems) (
      system: let
        overlays = [(import rust-overlay)];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        toolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;

        # buildInputs for Examples
        buildInputs = with pkgs; [
          stdenv.cc.cc.lib
          alsa-lib
          udev
          libxkbcommon
          libxkbcommon.dev
          wayland
          wayland-protocols
          xorg.libX11
          xorg.libXcursor
          xorg.libXrandr
          xorg.libXi
          vulkan-loader
        ];
      in {
        # `nix develop`
        devShells.default = pkgs.mkShell {
          packages = with pkgs;
            [
              toolchain
              pkg-config
              cargo-dist
              cargo-release
            ]
            ++ buildInputs;
          LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath buildInputs;
        };
      }
    );
}
