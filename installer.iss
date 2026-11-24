; MosaicDisk — Inno Setup 安装脚本（Windows x64）
; 编译：ISCC installer.iss /dAppVersion=0.1.0
; Inno Setup 7 及以上，中文语言文件随编译器附带。

#ifndef AppVersion
  #define AppVersion "0.1.0"
#endif
#define MyAppName "MosaicDisk"
#define MyAppPublisher "zeno-ling"
#define MyAppURL "https://github.com/zeno-ling/mosaicdisk"
#define MyAppExeName "mosaicdisk.exe"
#define MyAppIconName "app_icon.ico"

[Setup]
AppId={{2F4B9E1A-7C4D-4FF8-9A3B-6E2D8A1C0F55}}
AppName={#MyAppName}
AppVersion={#AppVersion}
AppVerName={#MyAppName} {#AppVersion}
AppPublisher={#MyAppPublisher}
AppPublisherURL={#MyAppURL}
AppSupportURL={#MyAppURL}/issues
AppUpdatesURL={#MyAppURL}/releases
DefaultDirName={autopf}\{#MyAppName}
DefaultGroupName={#MyAppName}
AllowNoIcons=yes
LicenseFile=LICENSE
OutputDir=target
OutputBaseFilename=MosaicDisk-{#AppVersion}-windows-x64-setup
SetupIconFile=assets\app_icon.ico
UninstallDisplayIcon={app}\{#MyAppIconName}
ArchitecturesInstallIn64BitMode=x64os
MinVersion=10.0
Compression=lzma2/max
SolidCompression=yes
WizardStyle=modern
DisableProgramGroupPage=yes

[Languages]
Name: "chinesesimplified"; MessagesFile: "compiler:Languages\ChineseSimplified.isl"
Name: "english"; MessagesFile: "compiler:Default.isl"

[CustomMessages]
chinesesimplified.ContextMenu=添加文件夹右键菜单「用 MosaicDisk 分析」
english.ContextMenu=Add "Analyze with MosaicDisk" to folder context menu

[Tasks]
Name: "desktopicon"; Description: "{cm:CreateDesktopIcon}"; GroupDescription: "{cm:AdditionalIcons}"; Flags: unchecked
Name: "contextmenu"; Description: "{cm:ContextMenu}"; GroupDescription: "{cm:AdditionalIcons}"; Flags: unchecked

[Files]
Source: "target\release\mosaicdisk.exe"; DestDir: "{app}"; Flags: ignoreversion
Source: "assets\app_icon.ico"; DestDir: "{app}"; Flags: ignoreversion
Source: "locale\*"; DestDir: "{app}\locale"; Flags: ignoreversion recursesubdirs createallsubdirs
Source: "README.md"; DestDir: "{app}"; Flags: ignoreversion
Source: "LICENSE"; DestDir: "{app}"; Flags: ignoreversion

[Icons]
Name: "{group}\{#MyAppName}"; Filename: "{app}\{#MyAppExeName}"; IconFilename: "{app}\{#MyAppIconName}"
Name: "{group}\{cm:UninstallProgram,{#MyAppName}}"; Filename: "{uninstallexe}"
Name: "{autodesktop}\{#MyAppName}"; Filename: "{app}\{#MyAppExeName}"; IconFilename: "{app}\{#MyAppIconName}"; Tasks: desktopicon

[Registry]
; 右键菜单：在文件夹上
Root: HKCR; Subkey: "directory\shell\MosaicDisk"; ValueType: string; ValueName: ""; ValueData: "用 MosaicDisk 分析"; Flags: uninsdeletekey; Tasks: contextmenu
Root: HKCR; Subkey: "directory\shell\MosaicDisk"; ValueType: string; ValueName: "Icon"; ValueData: "{app}\{#MyAppIconName}"; Flags: uninsdeletekey; Tasks: contextmenu
Root: HKCR; Subkey: "directory\shell\MosaicDisk\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#MyAppExeName}"" ""%1"""; Flags: uninsdeletekey; Tasks: contextmenu
; 右键菜单：在文件夹空白处
Root: HKCR; Subkey: "directory\background\shell\MosaicDisk"; ValueType: string; ValueName: ""; ValueData: "用 MosaicDisk 分析当前文件夹"; Flags: uninsdeletekey; Tasks: contextmenu
Root: HKCR; Subkey: "directory\background\shell\MosaicDisk"; ValueType: string; ValueName: "Icon"; ValueData: "{app}\{#MyAppIconName}"; Flags: uninsdeletekey; Tasks: contextmenu
Root: HKCR; Subkey: "directory\background\shell\MosaicDisk\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#MyAppExeName}"" ""%V"""; Flags: uninsdeletekey; Tasks: contextmenu

[Run]
Filename: "{app}\{#MyAppExeName}"; Description: "{cm:LaunchProgram,{#StringChange(MyAppName, '&', '&&')}}"; Flags: nowait postinstall skipifsilent
