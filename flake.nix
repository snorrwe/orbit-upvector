{
  description = "Bevy devshell";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-23.11";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = inputs@{ self, nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        bevy-deps = with pkgs; [
          udev
          alsa-lib
          xorg.libX11
          xorg.libXcursor
          xorg.libXi
          xorg.libXrandr # To use the x11 feature
          libxkbcommon
          libGL
          wayland # To use the wayland feature
          libz
          openssl
          vulkan-loader
          vulkan-extension-layer
          vulkan-validation-layers # don't need them *strictly* but immensely helpful
        ];
        tools = with pkgs; [
          pkg-config
          cargo-edit
          just
          parallel
        ];
      in
      with pkgs;
      {
        devShells.default = mkShell {
          buildInputs = [
            # rust deps
            mold
            llvmPackages_latest.clang
            stdenv
            (rust-bin.nightly.latest.default.override {
              extensions = [ "rust-src" "rust-analyzer" "rustfmt" ];
              targets = [ ];
            })
            python3
            python3Packages.sympy
          ] ++ bevy-deps ++ tools;
          LD_LIBRARY_PATH = lib.makeLibraryPath (bevy-deps);
        };
      }
    );
}

