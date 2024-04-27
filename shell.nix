{ pkgs ? import <nixpkgs> { } }:
let
  rust_overlay = import (builtins.fetchTarball
    "https://github.com/oxalica/rust-overlay/archive/master.tar.gz");
  pkgs = import <nixpkgs> { overlays = [ rust_overlay ]; };
  #rustVersion = "latest";
  rustVersion = "1.74.0";
  rust = pkgs.rust-bin.stable.${rustVersion}.default.override {
    extensions = [
      "rust-src" # for rust-analyzer
      "rust-analyzer"
    ];
  };
in pkgs.mkShell {
  buildInputs = [ rust ] ++ (with pkgs;
    [
      pkg-config
      # other dependencies
      #gtk3
      #wrapGAppsHook
    ]);
  RUST_BACKTRACE = 1;
}
