{ pkgs ? import <nixpkgs> {} }:
with pkgs;

stdenv.mkDerivation rec {
  name = "mzfmt";
  buildInputs = with pkgs; [
      clang_14
      openssl
      pkg-config
  ];
}
