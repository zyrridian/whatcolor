@echo off
setlocal
cd /d "%~dp0"

if not exist "target\release\whatcolor.exe" (
    cargo build --release || exit /b 1
)

start "" "target\release\whatcolor.exe"