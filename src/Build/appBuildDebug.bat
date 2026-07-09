@echo off
cd /d ..\App

cargo build

copy /Y .\target\debug\RustConnect.exe ..\build\RustConnectDebug.exe >nul

echo.
echo RustConnectDebug is now in ./build