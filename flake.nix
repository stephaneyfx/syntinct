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
          themeAttrs = {
            version = syntinct.version;
            buildInputs = [syntinct];
          };
          syntark = makeTheme { name = "syntark"; desc = "Dark neovim theme"; };
          thematic = makeTheme {
            name = "thematic";
            desc = "Dark and light neovim themes with a focus on semantics";
          };
          makeTheme = { name, desc }: pkgs.vimUtils.buildVimPlugin {
            pname = name;
            version = syntinct.version;
            src = pkgs.runCommand "${name}-src" themeAttrs ''
              mkdir -p $out/colors
              ${syntinct}/bin/syntinct generate ${name} > $out/colors/${name}.lua
            '';
            meta = {
              description = desc;
              homepage = "https://github.com/stephaneyfx/syntinct";
              license = pkgs.lib.licenses.bsd0;
            };
          };
        in {
          inherit syntark syntinct thematic;
        };
    in {
      overlays.default = final: prev: packagesFor final;
    } // flake-utils.lib.eachDefaultSystem (system: {
      packages = packagesFor nixpkgs.legacyPackages.${system};
    });
}
