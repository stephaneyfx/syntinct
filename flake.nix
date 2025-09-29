{
  description = "Color theme generator";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils/main";
  };

  outputs = {self, nixpkgs, flake-utils}:
    let
      packageDef = builtins.fromTOML (builtins.readFile ./Cargo.toml);
      packagesFor = pkgs:
        let
          system = pkgs.system;
          syntinct = pkgs.rustPlatform.buildRustPackage {
            pname = packageDef.package.name;
            version = packageDef.package.version;
            src = ./.;
            cargoLock.lockFile = ./Cargo.lock;
          };
          syntark-attrs = {
            version = syntinct.version;
            buildInputs = [syntinct];
          };
          syntark-src = pkgs.runCommand "syntark-src" syntark-attrs ''
            mkdir -p $out/colors
            ${syntinct}/bin/syntinct > $out/colors/syntark.lua
          '';
          syntark = pkgs.vimUtils.buildVimPlugin {
            pname = "syntark";
            version = syntinct.version;
            src = syntark-src;
            meta = {
              description = "Dark color theme for neovim";
              homepage = "https://github.com/stephaneyfx/syntinct";
              license = pkgs.lib.licenses.bsd0;
            };
          };
        in {
          inherit syntark syntinct;
        };
    in {
      overlays.default = final: prev: packagesFor final;
    } // flake-utils.lib.eachDefaultSystem (system: {
      packages = packagesFor nixpkgs.legacyPackages.${system};
    });
}
