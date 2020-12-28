let
  rust-version = "stable";

  nixpkgs = fetchGit {
    url = "https://github.com/patrickod/nixpkgs.git";
    rev = "458f2ae977948a8aaf930bed9af79eb69e76b7ae";
    ref = "personal";
  };

  mozilla-overlay = import (builtins.fetchTarball
    "https://github.com/mozilla/nixpkgs-mozilla/archive/master.tar.gz");

  pkgs = import nixpkgs { overlays = [ mozilla-overlay ]; };

  rust = pkgs.rustChannels.stable.rust.override {
    extensions = [ "rust-src" "clippy-preview" ];
    targets = [ "thumbv7em-none-eabihf" ];
  };

in pkgs.mkShell {
  name = "rust-dev";
  buildInputs =
    [ pkgs.probe-run pkgs.flip-link pkgs.gcc-arm-embedded pkgs.cargo-hf2 rust ];
}
