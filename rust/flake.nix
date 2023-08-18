{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    nixpkgs,
    rust-overlay,
    flake-utils,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (system: {
      devShells.default = let
        overlays = [(import rust-overlay)];
        pkgs = import nixpkgs {
          inherit overlays;
          inherit system;
        };
        rust-nightly = pkgs.rust-bin.selectLatestNightlyWith (toolchain:
          toolchain.default.override {
            extensions = ["rust-src"];
          });
      in
        with pkgs;
          pkgs.mkShell {
            buildInputs = [rust-nightly python310];
          };
    });
}
