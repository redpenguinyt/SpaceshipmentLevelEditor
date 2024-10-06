{
	description = "Flake for Playdate Development in Nix";

	inputs = { nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable"; };

  outputs = { self, nixpkgs, ... }:
	  let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.${system};
    in {
      devShells.x86_64-linux.default = pkgs.mkShell {
        nativeBuildInputs = with pkgs; [
          rustc cargo gcc
          SDL2
          SDL2_image
          SDL2_gfx
        ];

        shellHook = ''
          echo "Entered spaceshipment_editor dev environment"
        '';
      };
    };
}