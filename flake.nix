{
  description = "dimos — project scaffolding CLI, cross-compiled via musl";

  inputs = {
    nixpkgs.url      = "github:NixOS/nixpkgs/nixpkgs-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    rust-overlay.inputs.nixpkgs.follows = "nixpkgs";
    flake-utils.url  = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };

        # Pinned stable toolchain with musl cross-targets bundled.
        # Used both for the dev-shell and as the compiler in all nix builds.
        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          targets = [
            "x86_64-unknown-linux-musl"
            "aarch64-unknown-linux-musl"
          ];
        };

        # rustPlatform backed by our pinned toolchain.
        # buildRustPackage from this platform vendors crates via cargoLock,
        # so cargo never needs network access inside the sandbox.
        rustPlatform = pkgs.makeRustPlatform {
          cargo = rustToolchain;
          rustc = rustToolchain;
        };

        # Cross pkgs sets — used only to pull in the musl GCC cross-linkers.
        pkgsCrossX86 = import nixpkgs {
          inherit system;
          crossSystem.config = "x86_64-unknown-linux-musl";
        };
        pkgsCrossArm64 = import nixpkgs {
          inherit system;
          crossSystem.config = "aarch64-unknown-linux-musl";
        };

        linkerX86   = "${pkgsCrossX86.stdenv.cc}/bin/${pkgsCrossX86.stdenv.cc.targetPrefix}cc";
        linkerArm64 = "${pkgsCrossArm64.stdenv.cc}/bin/${pkgsCrossArm64.stdenv.cc.targetPrefix}cc";

        # Shared source + lock info for every build.
        commonArgs = {
          pname   = "dimos";
          version = "0.1.0";
          src     = ./.;
          cargoLock.lockFile = ./Cargo.lock;
          # Skip the test suite (musl cross builds can't run the test binary).
          doCheck = false;
        };

        # ── native macOS ARM build ──────────────────────────────────────────
        buildMacosArm = rustPlatform.buildRustPackage commonArgs;

        # ── generic cross build ─────────────────────────────────────────────
        # Overrides the build/install phases so cargo targets the right triple
        # and the binary is picked up from the target-specific output directory.
        buildCross = rustTarget: linker:
          rustPlatform.buildRustPackage (commonArgs // {
            pname = "dimos-${rustTarget}";

            buildPhase = ''
              runHook preBuild
              cargo build --release --target ${rustTarget}
              runHook postBuild
            '';

            installPhase = ''
              runHook preInstall
              mkdir -p $out/bin
              install -m755 target/${rustTarget}/release/dimos $out/bin/dimos
              runHook postInstall
            '';

            # Tell cargo which linker to use for the foreign target.
            "CARGO_TARGET_${
              builtins.replaceStrings ["-"] ["_"]
                (pkgs.lib.toUpper rustTarget)
            }_LINKER" = linker;
          });

      in {
        packages = {
          macos-arm   = buildMacosArm;
          linux-x86   = buildCross "x86_64-unknown-linux-musl"  linkerX86;
          linux-arm64 = buildCross "aarch64-unknown-linux-musl" linkerArm64;
          default     = buildMacosArm;
        };

        # Dev shell: all cross-linker env-vars pre-set so plain `cargo build
        # --target <triple>` works without any extra config.
        devShells.default = pkgs.mkShell {
          buildInputs = [ rustToolchain pkgs.pkg-config ];

          CARGO_TARGET_X86_64_UNKNOWN_LINUX_MUSL_LINKER  = linkerX86;
          CARGO_TARGET_AARCH64_UNKNOWN_LINUX_MUSL_LINKER = linkerArm64;

          shellHook = ''
            echo "dimos dev shell (Rust $(rustc --version | cut -d' ' -f2))"
            echo ""
            echo "  nix build .#macos-arm    -- macOS ARM (native)"
            echo "  nix build .#linux-x86   -- Linux x86_64 musl"
            echo "  nix build .#linux-arm64 -- Linux aarch64 musl"
            echo ""
            echo "  cargo build --release"
            echo "  cargo build --release --target x86_64-unknown-linux-musl"
            echo "  cargo build --release --target aarch64-unknown-linux-musl"
          '';
        };
      }
    );
}
