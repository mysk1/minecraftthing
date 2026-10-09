{
  lib,
  stdenv,
  rustPlatform,
}:

rustPlatform.buildRustPackage (finalAttrs: {
  pname = "numma";
  version = (fromTOML (builtins.readFile ../Cargo.toml)).package.version;

  src = ../.;

  cargoLock.lockFile = ../Cargo.lock;

  meta = {
    homepage = "https://github.com/mysk1/minecraftthing";
    description = "Update your minecraft mods with ease (WIP).";
    mainProgram = "numma";
    platforms = lib.platforms.linux;
    license = [
      lib.licenses.mit
      lib.licenses.asl20
    ];
    maintainers = [ lib.maintainers.choco98 ]; # and mysk0 :)
  };
})
