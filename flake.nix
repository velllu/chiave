{
  inputs = {
    naersk.url = "github:nix-community/naersk/master";
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-23.05";
    utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, utils, naersk }:
    utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        naersk-lib = pkgs.callPackage naersk { };
      in
      {
        defaultPackage = naersk-lib.buildPackage ./.;
        devShell = with pkgs; mkShell {
          buildInputs = [ cargo rustc rustfmt pre-commit rustPackages.clippy sqlx-cli sqlite ];
          RUST_SRC_PATH = rustPlatform.rustLibSrc;

          DATABASE_NAME = "./credentials.sqlite";
          DATABASE_URL = "sqlite:./credentials.sqlite";

          shellHook = ''
            bash -c "
              if [ -f '$DATABASE_NAME' ]; then
                echo -e '\e[34m[Chiave] Database already exists\e[0m'
              else
                echo -e '\e[34m[Chiave] Creating database\e[0m'
                sqlx database create
                sqlx migrate run
                rm "$DATABASE_NAME-shm"
                rm "$DATABASE_NAME-wal"
              fi
            "
          '';
        };
      });
}