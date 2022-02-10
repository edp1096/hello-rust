.\curl\bin\curl.exe --progress-bar -Lo rustup-init.exe https://static.rust-lang.org/rustup/dist/x86_64-pc-windows-msvc/rustup-init.exe

set RUSTUP_HOME=%cd%\rust\rustup
set CARGO_HOME=%cd%\rust\cargo

rustup-init.exe -y --no-modify-path --default-host x86_64-pc-windows-gnu --default-toolchain stable --profile minimal