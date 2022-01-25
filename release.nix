{ pkgs ? null } @ args:

let
  nixpkgs = builtins.fetchTarball {
    url = "https://github.com/nixos/nixpkgs/archive/f77036342e2b690c61c97202bf48f2ce13acc022.tar.gz";
    sha256 = "1vcrb2s6ngfv0vy7nwlqdqhy1whlrck3ws4ifk5dfhmvdy3jqmr4";
  };
  pkgs = if args ? "pkgs"
    then args.pkgs
    else (import nixpkgs { config = {}; });
in

with pkgs;

let
  rustupToolchain = "nightly-2021-10-06";
in

rec {
  shell = {
    dev = pkgs.mkShell rec {
      buildInputs = with pkgs; [
        rustup
        yq
        gnumake
      ];
      # Avoid polluting home dir with local project stuff.
      RUSTUP_HOME = toString ./.rustup;
      CARGO_HOME = toString ./.cargo;

      RUSTUP_TOOLCHAIN = rustupToolchain;

    # export PATH=$PATH:${RUSTUP_HOME}/toolchains/${rustupToolchain}-${rustBuildHostTriple}/bin/

      shellHook = ''
        export PATH=$PATH:${CARGO_HOME}/bin

        '';
      RUSTFLAGS = (builtins.map (a: ''-L ${a}/lib'') [
      ]);
    };
  };
}
