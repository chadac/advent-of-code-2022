{ name, rustPlatform, lib }:

rustPlatform.buildRustPackage {
  name = name;

  src = lib.cleanSource (./. + "/${name}");

  cargoLock.lockFile = ./. + "/${name}/Cargo.lock";
}
