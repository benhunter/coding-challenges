{
  description = "Rust development environment. Selects toolchain from ./rust-toolchain.toml. Supports Neovim LSP.";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        # Read the file relative to the flake's root
        overrides = (builtins.fromTOML (builtins.readFile (self + "/rust-toolchain.toml")));
        libPath = with pkgs; lib.makeLibraryPath [
          # load external libraries that you need in your rust project here
        ];
      in
      {
        devShells.default = pkgs.mkShell rec {
          # Additional build inputs to provide OpenSSL and pkg-config
          nativeBuildInputs = [
            pkgs.openssl
            pkgs.pkg-config
            pkgs.perl
            pkgs.rust-analyzer
            # pkgs.rust-analyzer-nightly
          ];

          buildInputs = with pkgs; [
            clang
            llvmPackages.bintools
            rustup
            cargo-generate
            openssl
          ];

          RUSTC_VERSION = overrides.toolchain.channel;
          
          # https://github.com/rust-lang/rust-bindgen#environment-variables
          LIBCLANG_PATH = pkgs.lib.makeLibraryPath [ pkgs.llvmPackages_latest.libclang.lib ];
          
          shellHook = ''
            export PATH=$PATH:''${CARGO_HOME:-~/.cargo}/bin
            export PATH=$PATH:''${RUSTUP_HOME:-~/.rustup}/toolchains/$RUSTC_VERSION-x86_64-unknown-linux-gnu/bin/
          '';

          # Add precompiled library to rustc search path
          RUSTFLAGS = (builtins.map (a: ''-L ${a}/lib'') [
            # add libraries here (e.g. pkgs.libvmi)
          ]) ++ ["--cfg openssl_vendored"];

          #RUSTFLAGS = "--cfg openssl_vendored";

          # Environment variables to help pkg-config locate OpenSSL
          # PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";
          # Environment variables to help the build process find OpenSSL
          # OPENSSL_LIB_DIR = "${pkgs.openssl.out}/lib";
          # OPENSSL_INCLUDE_DIR = "${pkgs.openssl.dev}/include";
          # DEP_OPENSSL_INCLUDE = "${pkgs.openssl.dev}/include";
          # OPENSSL_STATIC = 1;
          
          LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath (buildInputs ++ nativeBuildInputs);
          
          # Add glibc, clang, glib, and other headers to bindgen search path
          BINDGEN_EXTRA_CLANG_ARGS =
          # Includes normal include path
          (builtins.map (a: ''-I"${a}/include"'') [
            # add dev libraries here (e.g. pkgs.libvmi.dev)
            pkgs.glibc.dev
          ])
          # Includes with special directory paths
          ++ [
            ''-I"${pkgs.llvmPackages_latest.libclang.lib}/lib/clang/${pkgs.llvmPackages_latest.libclang.version}/include"''
            ''-I"${pkgs.glib.dev}/include/glib-2.0"''
            ''-I${pkgs.glib.out}/lib/glib-2.0/include/''
          ];
        };
      }
    );
}


# fenix rust overlay
#
# {
#   inputs = {
#     fenix = {
#       url = "github:nix-community/fenix";
#       inputs.nixpkgs.follows = "nixpkgs";
#     };
#     nixpkgs.url = "nixpkgs/nixos-unstable";
#   };
#
#   outputs = { self, fenix, nixpkgs }: {
#     packages.x86_64-linux.default = fenix.packages.x86_64-linux.minimal.toolchain;
#     nixosConfigurations.nixos = nixpkgs.lib.nixosSystem {
#       system = "x86_64-linux";
#       modules = [
#         ({ pkgs, ... }: {
#           nixpkgs.overlays = [ fenix.overlays.default ];
#           environment.systemPackages = with pkgs; [
#             (fenix.complete.withComponents [
#               "cargo"
#               "clippy"
#               "rust-src"
#               "rustc"
#               "rustfmt"
#             ])
#             rust-analyzer-nightly
#           ];
#         })
#       ];
#     };
#   };
# }

# oxalixa rust overlay
#
# {
#   description = "A devShell example";
#
#   inputs = {
#     nixpkgs.url      = "github:NixOS/nixpkgs/nixos-unstable";
#     rust-overlay.url = "github:oxalica/rust-overlay";
#     flake-utils.url  = "github:numtide/flake-utils";
#   };
#
#   outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
#     flake-utils.lib.eachDefaultSystem (system:
#       let
#         overlays = [ (import rust-overlay) ];
#         pkgs = import nixpkgs {
#           inherit system overlays;
#         };
#       in
#       {
#         devShells.default = with pkgs; mkShell {
#           buildInputs = [
#             openssl
#             pkg-config
#             eza
#             fd
#             # rust-bin.selectLatestNightlyWith (toolchain: toolchain.default) # or `toolchain.minimal`
#
#             rust-bin.nightly.latest.default
#             # rust-bin.beta.latest.default
#             rustup
#
#             cargo-nextest
#             cargo-watch
#             cargo-generate
#           ];
#
#           shellHook = ''
#             alias ls=eza
#             alias find=fd
#             # export FLAKE_SHELL="true"
#             # export IN_NIX_SHELL="pure" # Avoid prompt changes
#             # export NIX_SHELL_PRESERVE_PROMPT="true"
#           '';
#         };
#       }
#     );
# }
