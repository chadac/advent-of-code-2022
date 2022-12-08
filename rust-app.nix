{ name, src, rustPlatform, lib }:

rustPlatform.buildRustPackage {
  name = name;

  src = lib.cleanSource src;

  cargoLock.lockFile = "${src}/Cargo.lock";
}
