{
  description = "Rust development shell";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-25.11";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { 
    self, 
    nixpkgs,
    fenix,
  }: 
  let 
    system = "x86_64-linux";
    pkgs = import nixpkgs {
      inherit system;
      overlays = [fenix.overlays.default];
    };
    rust = fenix.packages.${system}.stable.withComponents [
      "cargo"
      "clippy"
      "rust-src"
      "rustc"
      "rustfmt"
    ];
  in
  {

    devShells.x86_64-linux.default = pkgs.mkShell {
      packages = [
        rust
        pkgs.pkg-config
        pkgs.libxkbcommon
        pkgs.vulkan-loader
        pkgs.wayland
        pkgs.udev
        pkgs.alsa-lib
      ];

      LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath [
        pkgs.wayland
        pkgs.libxkbcommon
        pkgs.vulkan-loader
        pkgs.udev
        pkgs.alsa-lib
      ];
    };
  };
}
