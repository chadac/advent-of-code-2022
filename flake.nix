{
  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    gitignore = {
      url = "github:hercules-ci/gitignore.nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, flake-utils, nixpkgs, gitignore}:
    let
      inherit (nixpkgs) lib;
      inherit (gitignore.lib) gitignoreSource;
      gitignoreOverlay = self: super: {
        lib = super.lib // {
          cleanSource = gitignoreSource;
        };
      };
    in
      flake-utils.lib.eachDefaultSystem (system:
        let
          pkgs = import nixpkgs {
            inherit system;
            overlays = [ gitignoreOverlay ];
          };
          problems = {
            day4 = pkgs.callPackage ./day4 { };
            day5 = pkgs.callPackage ./rust-app.nix {
              name = "day5";
              src = ./day5;
            };
            day8 = pkgs.callPackage ./rust-app.nix {
              name = "day8";
              src = ./day8;
            };
          };
        in
          {
            packages = problems // {
              default = pkgs.bash;
            };
            devShells.default = pkgs.mkShell {
              inputsFrom = (lib.attrValues problems);
              nativeBuildInputs = with pkgs; [
                cargo-edit
                cargo-fuzz
                rust-analyzer
              ];
            };
          }
      );
}
