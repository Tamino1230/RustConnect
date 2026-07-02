@echo off
cd /d ..\Server

go mod download
go build main.go

copy /Y main.exe ..\build\RustConnectServer.exe >nul

echo.
echo RustConnectServer is now in ./build