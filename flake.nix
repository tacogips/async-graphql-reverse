{
  description = "my rust project";

  inputs =
    {
      nixpkgs.url = "github:tacogips/nixpkgs-security/4a5495d1c0a7c62200b55deeeecb64f03b093056";
      flake-utils.url = "github:numtide/flake-utils";
      fenix =
        {
          url = "github:nix-community/fenix";
          inputs.nixpkgs.follows = "nixpkgs";
        };
      crane = {
        url = "github:ipetkov/crane/v0.24.0";
        inputs.nixpkgs.follows = "nixpkgs";
      };
    };

  outputs =
    { self
    , nixpkgs
    , flake-utils
    , fenix
    , crane
    ,
    }:
    flake-utils.lib.eachDefaultSystem (system:
    let
      overlays = [ fenix.overlays.default ];
      pkgs = import nixpkgs { inherit system overlays; };
      rust-components = fenix.packages.${system}.fromToolchainFile
        {
          file = ./rust-toolchain.toml;
          #sha256 = nixpkgs.lib.fakeSha256;
          sha256 = "sha256-ks0nMEGGXKrHnfv4Fku+vhQ7gx76ruv6Ij4fKZR3l78=";
        };

      crane-lib = (crane.mkLib pkgs).overrideToolchain rust-components;
      rust-src = crane-lib.cleanCargoSource (crane-lib.path ./.);

      crane-build-args = {
        src = rust-src;
        strictDeps = true;

        buildInputs = [
          # Add additional build inputs here
        ] ++ pkgs.lib.optionals pkgs.stdenv.isDarwin [
          pkgs.libiconv
        ];
      };
      root-cargo-artifacts = crane-lib.buildDepsOnly crane-build-args;

      root-crate = crane-lib.buildPackage {
        cargoArtifacts = root-cargo-artifacts;
        src = rust-src;
        cargoToml = ./Cargo.toml;
      };

      # TODO(tacogips) fix clippy warnings later
      #crate-clippy = crane-lib.cargoClippy {
      #  cargoArtifacts = root-cargo-artifacts;
      #  src = rust-src;
      #  #cargoClippyExtraArgs = "-- --deny warnings";
      #};

    in
    {
      # --- checks ---
      checks = {
        inherit root-crate; #crate-clippy;
      };
      packages.default = root-crate;

      apps.default = flake-utils.lib.mkApp {
        drv = root-crate;
      };

      formatter = pkgs.nixpkgs-fmt;

      # --- dev shell ---
      devShells.default = crane-lib.devShell
        {
          packages = with pkgs;
            [
              nixpkgs-fmt
              taplo
              cargo-make
              rust-analyzer
              cachix
            ];
          shellHook = ''
          '';

        };
    });
}
