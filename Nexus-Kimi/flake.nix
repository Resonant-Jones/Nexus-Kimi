{
  description = "ThreadSpace dev shell";
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  inputs.fenix.url = "github:nix-community/fenix";
  inputs.fenix.inputs.nixpkgs.follows = "nixpkgs";

  outputs = { self, nixpkgs, fenix }:
    let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.${system};
      rust = fenix.packages.${system}.stable;
      node = pkgs.nodejs_20;
    in
    {
      devShells.${system}.default = pkgs.mkShell {
        buildInputs = with pkgs; [
          just
          pkg-config
          openssl
          sqlite
          capnproto
          node
          rust
        ];
        shellHook = ''
          export RUST_LOG=debug
        '';
      };
    };
}