{
  pkgs ? import <nixpkgs> { },
}:

let
  # Import the flake for consistency
  flake = builtins.getFlake (toString ./.);

  # Get the development shell for the current system
  devShell = flake.devShells.${pkgs.system}.default;
in
devShell
