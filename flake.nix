{
  description = "Build a cargo project";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-24.11";

    crane.url = "github:ipetkov/crane";

    flake-utils.url = "github:numtide/flake-utils";

    advisory-db = {
      url = "github:rustsec/advisory-db";
      flake = false;
    };

    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      crane,
      flake-utils,
      advisory-db,
      fenix,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs { inherit system; };

        inherit (pkgs) lib;

        rustToolchain =
          with fenix.packages.${system};
          combine [
            stable.defaultToolchain
            (stable.withComponents [ "rust-src" ])

            rust-analyzer
          ];

        craneLib = (crane.mkLib pkgs).overrideToolchain rustToolchain;

        markdownFilter = path: _type: builtins.match ".*md$" path != null;
        markdownOrCargo = path: type: (markdownFilter path type) || (craneLib.filterCargoSources path type);

        src = lib.cleanSourceWith {
          src = ./.;
          filter = markdownOrCargo;
          name = "source";
        };

        # Build *just* the cargo dependencies, so we can reuse
        # all of that work (e.g. via cachix) when running in CI
        cargoArtifacts = craneLib.buildDepsOnly { inherit src; };

        # Build the actual crate itself, reusing the dependency
        # artifacts from above.
        libquassel = craneLib.buildPackage {
          inherit cargoArtifacts src;
        };
      in
      {
        checks =
          {
            # Build the crate as part of `nix flake check` for convenience
            inherit libquassel;

            # Run clippy (and deny all warnings) on the crate source,
            # again, resuing the dependency artifacts from above.
            #
            # Note that this is done as a separate derivation so that
            # we can block the CI if there are issues here, but not
            # prevent downstream consumers from building our crate by itself.
            libquassel-clippy = craneLib.cargoClippy {
              inherit cargoArtifacts src;
              cargoClippyExtraArgs = "--all-targets -- --deny warnings";
            };

            libquassel-doc = craneLib.cargoDoc { inherit cargoArtifacts src; };

            # Check formatting
            libquassel-fmt = craneLib.cargoFmt { inherit src; };

            # Audit dependencies
            libquassel-audit = craneLib.cargoAudit { inherit src advisory-db; };

            # Run tests with cargo-nextest
            # Consider setting `doCheck = false` on `libquassel` if you do not want
            # the tests to run twice
            libquassel-nextest = craneLib.cargoNextest {
              inherit cargoArtifacts src;
              partitions = 1;
              partitionType = "count";
            };
          }
          // lib.optionalAttrs (system == "x86_64-linux") {
            # NB: cargo-tarpaulin only supports x86_64 systems
            # Check code coverage (note: this will not upload coverage anywhere)
            libquassel-coverage = craneLib.cargoTarpaulin { inherit cargoArtifacts src; };
          };

        packages.default = libquassel;

        # apps.default = flake-utils.lib.mkApp {
        #   drv = libquassel;
        # };

        devShells.default = pkgs.mkShell {
          inputsFrom = builtins.attrValues self.checks;

          # Extra inputs can be added here
          nativeBuildInputs = with pkgs; [
            rustToolchain
            pkg-config
            glib
            cairo
            pango
            atk
            gdk-pixbuf
            gtk3
          ];
        };
      }
    );
}
