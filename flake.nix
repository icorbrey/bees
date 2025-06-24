{
  description = "Many hands make light work.";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  inputs.flake-utils.url = "github:numtide/flake-utils";

  outputs = { nixpkgs, flake-utils, ... }: flake-utils.lib.eachSystem
    ["x86_64-linux" "aarch64-linux" "x86_64-darwin"]
    (system: let
      pkgs = nixpkgs.legacyPackages.${system};
    in {
      devShells = import ./shell.nix {
        inherit pkgs;
      };
    });
}
