{
    description = "A flake for kubernetes postgresql controller.";

    inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-23.11";
    inputs.flake-utils.url = "github:numtide/flake-utils";

    outputs =
    {
        nixpkgs,
        flake-utils,
        ...
    }:
    let inherit (flake-utils.lib) eachDefaultSystem;
    in eachDefaultSystem(system: 
    let pkgs = nixpkgs.legacyPackages.${system};
    in {
        packages = rec {
            default = k8s-postgresql-controller;

            k8s-postgresql-controller = pkgs.rustPlatform.buildRustPackage {
                name = "postgresql-controller";
                src = ./.;

                cargoHash = "sha256-BPu28NpgWtHQITRPf58CqZeJX9HCuUmmVFEJ0Ijhgfc=";

                buildPhase = ''
                    cargo build --release
                '';

                installPhase = ''
                    mkdir -p $out/bin
                    install target/release/postgresql-controller -m 755 $out/bin/postgresql-controller
                '';

            };
            crdgen = pkgs.rustPlatform.buildRustPackage {
                name = "crdgen";

                src = ./.;

                cargoHash = "sha256-Ge+DCgjASxeEX3QTw9bDNK0Deo8bGi/asAyiIPvy034=";

                buildPhase = ''
                    cargo run --release --bin crdgen > crd.yaml
                '';

                installPhase = ''
                    mkdir -p $out/lib
                    cp crd.yaml $out/lib
                '';
            };
        };
    });
}
