let
  rust-version = "stable";

  nixpkgs = fetchGit {
    url = "https://github.com/patrickod/nixpkgs.git";
    rev = "8ec0706f5e6652ddb56a6370aaba80b90315b872";
    ref = "personal";
  };

  mozilla-overlay =
    import (builtins.fetchTarball https://github.com/mozilla/nixpkgs-mozilla/archive/master.tar.gz);

  pkgs = import nixpkgs {
    overlays = [ mozilla-overlay ];
  };

  rust = pkgs.rustChannels.stable.rust.override {
    extensions = [ "rust-src" ];
    targets = [
      "thumbv7em-none-eabihf"
    ];
  };

in
  pkgs.mkShell {
    name = "rust-dev";
    buildInputs = [
      rust
    ];
  }
