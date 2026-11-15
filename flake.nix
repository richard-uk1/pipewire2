{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        nixpkgs.follows = "nixpkgs";
      };
    };
  };
  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
      rust-overlay,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
        msrvRustVersion = (pkgs.lib.importTOML ./Cargo.toml).workspace.package.rust-version;
        # The rust-version in Cargo.toml is usually something like "1.88" but we need the full semantic version "1.88.0"
        rustOverlayVersion =
          if builtins.length (builtins.split "\\." msrvRustVersion) == 3 then
            "${msrvRustVersion}.0"
          else
            msrvRustVersion;
        rustToolchain = pkgs.rust-bin.stable."${rustOverlayVersion}".default.override {
          extensions = [
            "rustfmt"
            "rust-analyzer"
            "rust-src"
          ];
          targets = [
            "x86_64-unknown-linux-gnu"
            "wasm32-unknown-unknown"
            "x86_64-pc-windows-gnu"
            "aarch64-linux-android"
          ];
        };
      in
      {
        devShells.default = pkgs.mkShell rec {
          nativeBuildInputs = with pkgs; [
            clang
          ];
          buildInputs = with pkgs; [
            bashInteractive
            rustToolchain
            pkg-config

            fontconfig

            libxkbcommon
            libxcb
            libX11
            libXcursor
            libXi
            libXrandr
            libXxf86vm

            vulkan-loader

            wayland
            wayland-protocols
            wayland-scanner

            pipewire
            libclang
          ];
          LD_LIBRARY_PATH = "${pkgs.lib.makeLibraryPath buildInputs}";
          PIPEWIRE_CONFIG_PREFIX = "${pkgs.pipewire}/share/pipewire";
        };
      }
    );
}
