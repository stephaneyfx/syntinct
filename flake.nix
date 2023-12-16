{
  description = "Color theme generator";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils/main";

    naersk = {
      url = "github:nix-community/naersk/master";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, flake-utils, naersk }:
    let
      packagesFor = pkgs:
        let
          system = pkgs.system;
          buildRust = naersk.lib.${system}.buildPackage;
          syntinct = buildRust {
            root = self;
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
              license = pkgs.lib.licenses.mit;
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
