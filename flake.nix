{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, utils }:
    utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
      in
      {
        defaultPackage = pkgs.rustPlatform.buildRustPackage rec {
          pname = "mpi-traffic";
          version = "0.0.0";

          src = ./.;

          cargoSha256 = "sha256-xwxqzaz7n7S3NrBRn6671EfzfDSEr8sx8lj2kTVVTL0=";
          verifyCargoDeps = true;

          RUST_BACKTRACE = 1;
          LIBCLANG_PATH = "${pkgs.llvmPackages.libclang}/lib";
          nativeBuildInputs = with pkgs; [
            autoconf
            automake
            libtool
            pkg-config
            clang
            texinfo
          ];
          buildInputs = with pkgs; [
            mpich
            wayland
            libglvnd
          ] ++ (with pkgs.xorg; [
            libX11
            libXcursor
            libXrandr
            libXi
          ]);

          postFixup =
            let
              rpath = pkgs.lib.makeLibraryPath buildInputs;
            in
            ''
              patchelf --set-rpath ${rpath} $out/bin/*
            '';

          meta = with pkgs.stdenv.lib; {
            description = "Traffic simulation with MPI";
            homepage = https://github.com/linyinfeng/mpi-traffic;
            license = licenses.mit;
          };
        };
      });
}
