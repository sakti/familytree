{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        nixpkgs.follows = "nixpkgs";
        flake-utils.follows = "flake-utils";
      };
    };
    crane = {
      url = "github:ipetkov/crane";
      inputs = {
        nixpkgs.follows = "nixpkgs";
        rust-overlay.follows = "rust-overlay";
        flake-utils.follows = "flake-utils";
      };
    };
  };
  outputs = { self, nixpkgs, flake-utils, rust-overlay, crane }:
    flake-utils.lib.eachDefaultSystem
      (system:
        let
          overlays = [ (import rust-overlay) ];
          pkgs = import nixpkgs {
            inherit system overlays;
          };
          rustToolchain = pkgs.pkgsBuildHost.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;

          craneLib = (crane.mkLib pkgs).overrideToolchain rustToolchain;
          src = craneLib.cleanCargoSource ./.;

          nativeBuildInputs = with pkgs; [ rustToolchain pkg-config ];

         # if pkgs.stdenv.isDarwin then { 
         #     buildInputs = with pkgs; [openssl darwin.apple_sdk.frameworks.Security];
         # } else { 
         #     buildInputs = with pkgs;  [openssl];
         # };
          depsBuildBuild = with pkgs; [ clang llvm ];
          buildInputs = with pkgs; [openssl (lib.optional stdenv.isDarwin darwin.apple_sdk.frameworks.Security)];

          commonArgs = {
            inherit src buildInputs nativeBuildInputs depsBuildBuild;
          };
          cargoArtifacts = craneLib.buildDepsOnly commonArgs;

          bin = craneLib.buildPackage (commonArgs // {
            inherit cargoArtifacts;

            LIBCLANG_PATH = "${pkgs.llvmPackages.libclang.lib}/lib";
          });
          dockerImage = pkgs.dockerTools.streamLayeredImage {
            name = "familytree";
            tag = "latest";
            contents = [ bin ];
            config = {
              Cmd = [ "${bin}/bin/familytree" ];
            };
          };
        in
        with pkgs;
        {
          packages =
            {
			  # that way we can build `bin` specifically,
			  # but it's also the default.
              inherit bin dockerImage;
              default = bin;
            };
          devShells.default = mkShell {
            inputsFrom = [ bin ];
            buildInputs = with pkgs; [ dive ];
          };
        }
      );
}
