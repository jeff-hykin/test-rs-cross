{
  description = "Rust hello world with musl cross-compilation";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    rust-overlay.inputs.nixpkgs.follows = "nixpkgs";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };

        # Rust toolchain with all cross-compilation targets
        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          targets = [
            "x86_64-unknown-linux-musl"
            "aarch64-unknown-linux-musl"
            # macOS ARM is the native host on aarch64-apple-darwin
          ];
        };

        # Cross-compilation packages for Linux targets
        pkgsCrossLinuxX86 = import nixpkgs {
          inherit system overlays;
          crossSystem = {
            config = "x86_64-unknown-linux-musl";
          };
        };

        pkgsCrossLinuxArm64 = import nixpkgs {
          inherit system overlays;
          crossSystem = {
            config = "aarch64-unknown-linux-musl";
          };
        };

        # Get cross-linkers from cross pkgs
        linkerX86   = "${pkgsCrossLinuxX86.stdenv.cc}/bin/${pkgsCrossLinuxX86.stdenv.cc.targetPrefix}cc";
        linkerArm64 = "${pkgsCrossLinuxArm64.stdenv.cc}/bin/${pkgsCrossLinuxArm64.stdenv.cc.targetPrefix}cc";

        # Helper: build for a given rust target using a specific linker
        buildFor = { rustTarget, linker, extraEnv ? {} }:
          pkgs.stdenv.mkDerivation ({
            name = "test-rs-cross-${rustTarget}";
            src = ./.;

            nativeBuildInputs = [ rustToolchain pkgs.pkg-config ];

            CARGO_BUILD_TARGET = rustTarget;
            "CARGO_TARGET_${builtins.replaceStrings ["-"] ["_"] (pkgs.lib.toUpper rustTarget)}_LINKER" = linker;

            buildPhase = ''
              export HOME=$TMPDIR
              cargo build --release --target ${rustTarget}
            '';

            installPhase = ''
              mkdir -p $out/bin
              cp target/${rustTarget}/release/test-rs-cross $out/bin/test-rs-cross
            '';
          } // extraEnv);

        # macOS ARM native build (no cross-compilation needed)
        buildMacosArm = pkgs.stdenv.mkDerivation {
          name = "test-rs-cross-aarch64-apple-darwin";
          src = ./.;
          nativeBuildInputs = [ rustToolchain ];
          buildPhase = ''
            export HOME=$TMPDIR
            cargo build --release
          '';
          installPhase = ''
            mkdir -p $out/bin
            cp target/release/test-rs-cross $out/bin/test-rs-cross
          '';
        };

      in {
        packages = {
          # macOS ARM (native)
          macos-arm = buildMacosArm;

          # Linux x86_64 musl
          linux-x86 = buildFor {
            rustTarget = "x86_64-unknown-linux-musl";
            linker = linkerX86;
          };

          # Linux aarch64 musl
          linux-arm64 = buildFor {
            rustTarget = "aarch64-unknown-linux-musl";
            linker = linkerArm64;
          };

          default = buildMacosArm;
        };

        # Dev shell with all tools available for manual cargo cross commands
        devShells.default = pkgs.mkShell {
          buildInputs = [ rustToolchain pkgs.pkg-config ];

          # Expose cross-linkers as environment variables
          CARGO_TARGET_X86_64_UNKNOWN_LINUX_MUSL_LINKER = linkerX86;
          CARGO_TARGET_AARCH64_UNKNOWN_LINUX_MUSL_LINKER = linkerArm64;

          shellHook = ''
            echo "Rust cross-compilation dev shell"
            echo ""
            echo "Available build commands:"
            echo "  nix build .#macos-arm    -- macOS ARM (native)"
            echo "  nix build .#linux-x86   -- Linux x86_64 musl"
            echo "  nix build .#linux-arm64 -- Linux aarch64 musl"
            echo ""
            echo "Or use cargo directly inside this shell:"
            echo "  cargo build --release"
            echo "  cargo build --release --target x86_64-unknown-linux-musl"
            echo "  cargo build --release --target aarch64-unknown-linux-musl"
          '';
        };
      }
    );
}
