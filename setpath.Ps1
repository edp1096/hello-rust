# If cannot run by execution policy, run follow command
# Set-ExecutionPolicy -Scope CurrentUser Unrestricted

Set-PSDebug -Off

$env:RUSTUP_HOME = $pwd.Path + "\rust\rustup"
$env:CARGO_HOME = $pwd.Path + "\rust\cargo"

$env:Path += ";" + $pwd.Path + "\rust\cargo\bin"
