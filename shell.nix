# save this as shell.nix
{ pkgs ? import <nixpkgs> {}}:

pkgs.mkShell {
  buildInputs = with pkgs; [
    pkg-config
    cargo-tauri
    dbus
    openssl_3
    glib
    gtk3
    libsoup
    webkitgtk
    appimagekit
    alsa-lib
    rustc
    cargo
    gcc
    rustfmt
    clippy
  ];

  RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
}
