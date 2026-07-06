
<p align="center"><img src="docs/RustConnectIcon.png" width="128" alt="MAS Logo"></p>

<h1 align="center">RustConnect</h1>

<p align="center">
  A written in Rust Open-Source Smooth 60fps Screensharing App with a interactive Pipe: rustconnect://CODE. Official webservice API isnt implemented yet!
</p>

<hr>

<p align="center">
    <a target="_blank" href="https://github.com/Tamino1230/RustConnect/releases/latest/download/RustConnectInstaller.exe">
        <img alt="Website" src="https://img.shields.io/badge/windows-instant_download_link-green">
    </a>
    <a target="_blank" href="https://github.com/Tamino1230/RustConnect/releases/latest/download/RustConnect-Linux.zip">
        <img alt="Website" src="https://img.shields.io/badge/linux-instant_download_link-blue">
    </a>
</p>

<p align="center">
    <img alt="Website" src="https://img.shields.io/website?url=https%3A%2F%2Frustconnectserver.onrender.com&up_message=Currently%20Up&down_message=In%20sleepmode&label=RC%20API">
    <img alt="GitHub Actions Workflow Status" src="https://img.shields.io/github/actions/workflow/status/Tamino1230/RustConnect/linux_build.yml?label=Linux%20Build">
    <img alt="GitHub Actions Workflow Status" src="https://img.shields.io/github/actions/workflow/status/Tamino1230/RustConnect/build.yml?label=Windows%20Build">
    <img alt="GitHub Downloads (all assets, all releases)" src="https://img.shields.io/github/downloads/Tamino1230/RustConnect/total">
</p>

<p align="center">
    <p>Help is veryyyy welcome here :3</p>
    <img src="docs/connection.png">
    <img height="360px" src="docs/stream.png">
</p>

> [!CAUTION]
> Experimental Webserver! For localhost / exposing port change to localhost in DevBuild.
>
> Due to Server-limitation is the Screensharing not smooth!

## Installation
Go to the [Latest Release](https://github.com/Tamino1230/RustConnect/releases/latest) and download the `RustConnectionInstaller.exe` and run it!

## Making Dev Build

### Windows
If you wanna edit code and make a dev build run:
```sh
# on windows
git clone https://github.com/Tamino1230/RustConnect.git
cd RustConnect/src/build/
.\build.bat 

# on linux 
chmod +x linuxAppBuild.sh
linuxAppBuild.sh
```
then the **RustConnect.exe** will appear in the build folder.