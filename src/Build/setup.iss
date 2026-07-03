[Setup]
AppName=RustConnect
AppVersion=1.0
DefaultDirName={pf}\RustConnect
DefaultGroupName=RustConnect
SetupIconFile=icon.ico
UninstallDisplayIcon=icon.ico
OutputDir=Output
OutputBaseFilename=RustConnectInstaller
Compression=lzma
SolidCompression=yes

[Files]
Source: "RustConnect.exe"; DestDir: "{app}"; Flags: ignoreversion
Source: "icon.ico"; DestDir: "{app}"; Flags: ignoreversion

[Icons]
Name: "{group}\RustConnect"; Filename: "{app}\RustConnect.exe"
Name: "{commondesktop}\RustConnect"; Filename: "{app}\RustConnect.exe"; Tasks: desktopicon; IconFilename: "icon.ico"

[Tasks]
Name: "desktopicon"; Description: "Create a desktop icon"; GroupDescription: "Additional icons:"; Flags: unchecked