; Created by "innounp" version 2.71.4
; Setup file: MosaicDisk-0.1.0-windows-x64-setup.exe
; Inno Setup Version: 7.1.1 (Unicode)

[Setup]
AppName=MosaicDisk
AppVerName=MosaicDisk 0.1.0
AppId={{2F4B9E1A-7C4D-4FF8-9A3B-6E2D8A1C0F55}}
AppVersion=0.1.0
AppPublisher=zeno-ling
AppPublisherURL=https://github.com/zeno-ling/mosaicdisk
AppSupportURL=https://github.com/zeno-ling/mosaicdisk/issues
AppUpdatesURL=https://github.com/zeno-ling/mosaicdisk/releases
DefaultDirName={autopf}\MosaicDisk
DefaultGroupName=MosaicDisk
OutputBaseFilename=MosaicDisk-0.1.0-windows-x64-setup
UninstallDisplayIcon={app}\mosaicdisk.exe
Compression=lzma2
ArchitecturesAllowed=x86compatible
ArchitecturesInstallIn64BitMode=x64os
DisableDirPage=auto
DisableProgramGroupPage=yes
ChangesAssociations=no
ShowLanguageDialog=yes
AllowNoIcons=yes
LicenseFile=embedded\License.txt
WizardStyle=modern light
WizardImageFile=embedded\WizardImage0.png
WizardSmallImageFile=embedded\WizardSmallImage0.png
;TimeStampsInUTC=yes

[Files]
Source: "{app}\mosaicdisk.exe"; DestDir: "{app}"; Flags: ignoreversion 
Source: "{app}\locale\zh_CN\LC_MESSAGES\default.po"; DestDir: "{app}\locale\zh_CN\LC_MESSAGES"; Flags: ignoreversion createallsubdirs 
Source: "{app}\README.md"; DestDir: "{app}"; Flags: ignoreversion 
Source: "{app}\LICENSE"; DestDir: "{app}"; Flags: ignoreversion 

[Registry]
Root: HKCR; Subkey: "directory\shell\MosaicDisk"; ValueType: String; ValueData: "用 MosaicDisk 分析"; Tasks: contextmenu; Flags: uninsdeletekey 
Root: HKCR; Subkey: "directory\shell\MosaicDisk"; ValueName: "Icon"; ValueType: String; ValueData: "{app}\mosaicdisk.exe"; Tasks: contextmenu; Flags: uninsdeletekey 
Root: HKCR; Subkey: "directory\shell\MosaicDisk\command"; ValueType: String; ValueData: """{app}\mosaicdisk.exe"" ""%1"""; Tasks: contextmenu; Flags: uninsdeletekey 
Root: HKCR; Subkey: "directory\background\shell\MosaicDisk"; ValueType: String; ValueData: "用 MosaicDisk 分析当前文件夹"; Tasks: contextmenu; Flags: uninsdeletekey 
Root: HKCR; Subkey: "directory\background\shell\MosaicDisk"; ValueName: "Icon"; ValueType: String; ValueData: "{app}\mosaicdisk.exe"; Tasks: contextmenu; Flags: uninsdeletekey 
Root: HKCR; Subkey: "directory\background\shell\MosaicDisk\command"; ValueType: String; ValueData: """{app}\mosaicdisk.exe"" ""%V"""; Tasks: contextmenu; Flags: uninsdeletekey 

[Run]
Filename: "{app}\mosaicdisk.exe"; Description: "{cm:LaunchProgram,MosaicDisk}"; Flags: postinstall skipifsilent runasoriginaluser nowait

[Icons]
Name: "{group}\MosaicDisk"; Filename: "{app}\mosaicdisk.exe"; 
Name: "{group}\{cm:UninstallProgram,MosaicDisk}"; Filename: "{uninstallexe}"; 
Name: "{autodesktop}\MosaicDisk"; Filename: "{app}\mosaicdisk.exe"; Tasks: desktopicon; 

[Tasks]
Name: "desktopicon"; Description: "{cm:CreateDesktopIcon}"; GroupDescription: "{cm:AdditionalIcons}"; Flags: unchecked 
Name: "contextmenu"; Description: "{cm:ContextMenu}"; GroupDescription: "{cm:AdditionalIcons}"; Flags: unchecked 

[CustomMessages]
chinesesimplified.NameAndVersion=%1 版本 %2
chinesesimplified.AdditionalIcons=附加快捷方式：
chinesesimplified.CreateDesktopIcon=创建桌面快捷方式(&D)
chinesesimplified.CreateQuickLaunchIcon=创建快速启动栏快捷方式(&Q)
chinesesimplified.ProgramOnTheWeb=%1 网站
chinesesimplified.UninstallProgram=卸载 %1
chinesesimplified.LaunchProgram=运行 %1
chinesesimplified.AssocFileExtension=将 %2 文件扩展名与 %1 建立关联(&A)
chinesesimplified.AssocingFileExtension=正在将 %2 文件扩展名与 %1 建立关联...
chinesesimplified.AutoStartProgramGroupDescription=启动：
chinesesimplified.AutoStartProgram=自动启动 %1
chinesesimplified.AddonHostProgramNotFound=您选择的文件夹中无法找到 %1。%n%n您确定要继续吗？
english.NameAndVersion=%1 version %2
english.AdditionalIcons=Additional shortcuts:
english.CreateDesktopIcon=Create a &desktop shortcut
english.CreateQuickLaunchIcon=Create a &Quick Launch shortcut
english.ProgramOnTheWeb=%1 on the Web
english.UninstallProgram=Uninstall %1
english.LaunchProgram=Launch %1
english.AssocFileExtension=&Associate %1 with the %2 file extension
english.AssocingFileExtension=Associating %1 with the %2 file extension...
english.AutoStartProgramGroupDescription=Startup:
english.AutoStartProgram=Automatically start %1
english.AddonHostProgramNotFound=%1 could not be located in the folder you selected.%n%nDo you want to continue anyway?
chinesesimplified.ContextMenu=添加文件夹右键菜单「用 MosaicDisk 分析」
english.ContextMenu=Add "Analyze with MosaicDisk" to folder context menu

[Languages]
; These files are stubs
; To achieve better results after recompilation, use the real language files
Name: "chinesesimplified"; MessagesFile: "embedded\chinesesimplified.isl"; 
Name: "english"; MessagesFile: "embedded\english.isl"; 
