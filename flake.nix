{
  description = "tracer-tong";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      flake = false;
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }: flake-utils.lib.eachDefaultSystem (system:
    let
      rust-overlay' = import rust-overlay;
      pkgs = import nixpkgs {
        inherit system;
        overlays = [ rust-overlay' ];
      };
      rust-stable = pkgs.rustChannelOf { channel = "stable"; };
    in {
      
      devShell = pkgs.mkShell {
        buildInputs = [ ];
        nativeBuildInputs = [ rust-stable.rust ];
      };
      
    });
}
