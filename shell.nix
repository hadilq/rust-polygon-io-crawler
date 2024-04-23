{ pkgs ? import <nixpkgs> {} }:
let
  userHome = "${builtins.toString ./.user-home}";

  localConfig = import ./.config.nix;
in
pkgs.mkShell {
  name = "rust-experiment";
  buildInputs = with pkgs; [
    rustc
    cargo
    cargo-flamegraph
    cargo-release
    openssl
    pkg-config
    # rustup
  ];

  RUST_BACKTRACE = "full";

  LANG = "C.UTF-8";
  LC_ALL = "C.UTF-8";

  CARGO_HOME = "${userHome}/.cargo";
  RUSTUP_HOME = "${userHome}/.rustup";
  # HOME = "${userHome}";
  # USER_HOME = "${userHome}";
  DOCKER_BUILDKIT = "1";
  DB_USER = "${localConfig.dbUser}";
  DB_PASSWORD = "${localConfig.dbPassword}";
  API_KEY = "${localConfig.apiKey}";
  DB_NAME = "${localConfig.dbName}";
  DB_DATA = "${localConfig.dbData}";
  DB_TEST_NAME = "${localConfig.dbTestName}";
  DB_TEST_DATA = "${localConfig.dbTestData}";
  DB_PORT = "${toString localConfig.dbPort}";
  POLICY_COOL_DOWN_TIME = "${toString localConfig.policyCoolDownTime}";
  POLICY_ALLOWED_REQUESTS = "${toString localConfig.policyAllowedRequests}";

  shellHook = ''
    # Write out local.properties for Android Studio.
    cat <<EOF > container/.env
    DB_USER='${localConfig.dbUser}'
    DB_PASSWORD='${localConfig.dbPassword}'
    DB_NAME='${localConfig.dbName}'
    DB_DATA='${localConfig.dbData}'
    DB_TEST_NAME='${localConfig.dbTestName}'
    DB_TEST_DATA='${localConfig.dbTestData}'
    DB_PORT=${toString localConfig.dbPort}
    EOF
  '';
}
