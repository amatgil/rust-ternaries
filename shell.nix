{
  pkgs ? import <nixpkgs> { },
  lib ? pkgs.lib,
}:
let
  packages = with pkgs; [
    rust-analyzer
    rustfmt
    mold
    rust-bin.stable.latest.default
  ];
in
pkgs.mkShell {
  # Get dependencies from the main package
  inputsFrom = [ (pkgs.callPackage ./default.nix { }) ];
  nativeBuildInputs = packages;
  buildInputs = packages;
  env = {
    LIBCLANG_PATH = "${pkgs.libclang.lib}/lib";
    LD_LIBRARY_PATH = "${lib.makeLibraryPath packages}";
  };
}
