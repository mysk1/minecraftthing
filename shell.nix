let
  sources = import ./lon.nix;
  pkgs = import sources.nixpkgs { };
in
pkgs.mkShell {
  packages = with pkgs; [
    lon # for pins
    nixfmt-tree

    rustc
    cargo
    clippy
    rustfmt
    rust-analyzer
  ];
}
