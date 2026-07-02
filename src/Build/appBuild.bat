@echo off
cd /d ..\App

cargo build --release

copy /Y .\target\release\RustConnect.exe ..\build\RustConnect.exe >nul

echo.
echo RustConnect is now in ./build