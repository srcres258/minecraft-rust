# SPDX-License-Identifier: Apache-2.0
# Copyright (c) 2024 minecraft-rust contributors.
#
# flake.nix — Nix development shell for minecraft-rust
#
# Usage:
#   nix develop
#   cargo build
#   cargo run

{
  description = "minecraft-rust — Minecraft clone in Rust with SFML";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-25.11";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
      in
      {
        devShells.default = pkgs.mkShell {
          name = "minecraft-rust-dev";

          nativeBuildInputs = with pkgs; [
            # Build-time tooling
            pkg-config
            cmake
            gcc
            binutils
          ];

          buildInputs = with pkgs; [
            # Rust toolchain
            cargo
            rustc
            rustfmt
            clippy
            rust-analyzer

            # CSFML 2.6.1 — C bindings for SFML.
            # Internally depends on SFML 2.6.2 (not the top-level "sfml"
            # which is 3.0.x).  The Rust "sfml" crate (0.21.0) wraps
            # CSFML 2.6, so this is the correct match.
            csfml

            # SFML 2.6.2 — C++ library that CSFML wraps.
            # Explicitly included so pkg-config can resolve transitive
            # sfml-*.pc files if needed.
            sfml_2
          ];

          # Runtime library search path.
          #
          # The Rust binary needs to find libcsfml-*.so at runtime.
          # CSFML's .so files have RUNPATH set to SFML 2.6.2's lib dir,
          # and SFML's .so files have RUNPATH set to their own deps
          # (X11, OpenGL, udev, freetype, etc.), so the full chain
          # resolves from a single LD_LIBRARY_PATH entry.
          #
          # We also include libGL explicitly — the "gl" and "gl_loader"
          # crates load it via dlopen at runtime, which first searches
          # LD_LIBRARY_PATH.
          LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath [
            pkgs.csfml
            pkgs.sfml_2
            pkgs.libGL
          ];

          shellHook = ''
            export RUST_BACKTRACE=1
            echo "🦀 minecraft-rust dev shell"
            echo "   cargo build  — build the project"
            echo "   cargo run    — build and run the game"
          '';
        };
      });
}
