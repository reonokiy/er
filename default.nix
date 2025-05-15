{
  pkgs ? import <nixpkgs> { },
}:

let
  # Import the flake for consistency
  flake = builtins.getFlake (toString ./.);

  # Get the default package for the current system
  package = flake.packages.${pkgs.system}.default;
in
package
