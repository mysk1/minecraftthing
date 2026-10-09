let
  sources = import ./lon.nix;
  pkgs = import sources.nixpkgs { };
in
(pkgs.callPackage ./nix/package.nix { })
