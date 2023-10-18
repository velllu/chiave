{ pkgs, ... }:

{
  packages = [
    pkgs.sqlx-cli
  ];

  env.DATABASE_NAME = "./credentials.sqlite";
  env.DATABASE_URL = "sqlite:./credentials.sqlite";

  scripts.create-db.exec = ''
    sqlx database create
    sqlx migrate run
  '';

  enterShell = ''
    bash -c "
      if [ -f '$DATABASE_NAME' ]; then
        echo -e '\e[34m[Chiave] Database already exists\e[0m'
      else
        echo -e '\e[34m[Chiave] Creating database\e[0m'
        create-db
      fi
    "
  '';

  languages.rust = {
    enable = true;
    channel = "stable";

    components = [ "rustc" "cargo" "clippy" "rustfmt" "rust-analyzer" ];
  };

  pre-commit.hooks = {
    rustfmt.enable = true;
    clippy.enable = true;
  };
}
