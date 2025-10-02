{
  description = "wasm + wgpu + Rust dev environment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
  }:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = import nixpkgs {inherit system;};
    in {
      devShells.default = pkgs.mkShell rec {
        buildInputs = with pkgs; [
          nodejs
          pkg-config
          gobject-introspection
          openssl
          # Extra dependencies
          lld

          # cargo packages

          cargo
          cargo-watch
          cargo-modules
          # wasm dependencies
          wasm-pack
          binaryen # useful for wasm-opt
          nodejs
          pnpm # or yarn/npm, whichever you like

          # necessary for building wgpu in 3rd party packages (in most cases)
          libxkbcommon
          wayland
          xorg.libX11
          xorg.libXcursor
          xorg.libXrandr
          xorg.libXi
          alsa-lib
          fontconfig
          freetype
          shaderc
          directx-shader-compiler
          pkg-config
          cmake
          mold # could use any linker, needed for rustix (but mold is fast)

          libGL
          vulkan-headers
          vulkan-loader
          vulkan-tools
          vulkan-tools-lunarg
          vulkan-extension-layer
          vulkan-validation-layers # don't need them *strictly* but immensely helpful

          # necessary for developing (all of) wgpu itself
          cargo-nextest
          cargo-fuzz

          # nice for developing wgpu itself
          typos

          # if you don't already have rust installed through other means,
          # this shell.nix can do that for you with this below
          yq # for tomlq below
          rustup

          # nice tools
          gdb
          rr
          evcxr
          valgrind
          renderdoc
        ];

        shellHook = ''
          echo "🚀 Entered chaos shapes shell💥"
          export PKG_CONFIG_PATH=$PKG_CONFIG_PATH:${pkgs.librsvg.dev}/lib/pkgconfig
          export RUSTC_VERSION="$(tomlq -r .toolchain.channel rust-toolchain.toml)"
          export PATH="$PATH:''${CARGO_HOME:-~/.cargo}/bin"
          export PATH="$PATH:''${RUSTUP_HOME:-~/.rustup/toolchains/$RUSTC_VERSION-x86_64-unknown-linux/bin}"
          export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:${builtins.toString (pkgs.lib.makeLibraryPath buildInputs)}";
          rustup default $RUSTC_VERSION
          rustup component add rust-src rust-analyzer

                export SHELL=$(which zsh)   # or zsh
        '';
      };
    });
}
