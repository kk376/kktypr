{
  description = "A blazingly fast touch typing academy and developer syntax tutor for your terminal in pure Rust";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs = { self, nixpkgs }:
    let
      supportedSystems = [
        "x86_64-linux"
        "aarch64-linux"
        "x86_64-darwin"
        "aarch64-darwin"
      ];
      forAllSystems = nixpkgs.lib.genAttrs supportedSystems;
      nixpkgsFor = forAllSystems (system: import nixpkgs { inherit system; });
    in
    {
      packages = forAllSystems (system: {
        kktypr = nixpkgsFor.${system}.callPackage ./packaging/nix/package.nix {
          src = nixpkgs.lib.cleanSource ./.;
        };
        default = self.packages.${system}.kktypr;
      });

      apps = forAllSystems (system: {
        kktypr = {
          type = "app";
          program = "${self.packages.${system}.kktypr}/bin/kktypr";
        };
        default = self.apps.${system}.kktypr;
      });

      devShells = forAllSystems (system: {
        default = nixpkgsFor.${system}.mkShell {
          inputsFrom = [ self.packages.${system}.kktypr ];
          packages = with nixpkgsFor.${system}; [
            cargo
            rustc
            rustfmt
            clippy
            rust-analyzer
          ];
        };
      });

      overlays.default = final: prev: {
        kktypr = final.callPackage ./packaging/nix/package.nix {
          src = final.lib.cleanSource ./.;
        };
      };
    };
}
