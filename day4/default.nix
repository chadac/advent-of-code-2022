{ rustPlatform, lib }:

rustPlatform.buildRustPackage {
  name = "day4";

  src = lib.cleanSource ./.;

  cargoLock.lockFile = ./Cargo.lock;
}
