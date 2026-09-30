#ifndef AppVersion
  #define AppVersion "1.0.1"
#endif
#ifndef PackageDir
  #define PackageDir "..\dist\portable"
#endif

[Setup]
AppId={{D5CB2863-B795-432C-BF82-EC3726A30DF4}
AppName=Workshop Device
AppVersion={#AppVersion}
AppPublisher=whisperbtw
AppPublisherURL=https://github.com/whisperbtw/workshop-device
AppSupportURL=https://github.com/whisperbtw/workshop-device/issues
AppUpdatesURL=https://github.com/whisperbtw/workshop-device/releases
DefaultDirName={localappdata}\Programs\Workshop Device
DefaultGroupName=Workshop Device
PrivilegesRequired=lowest
ArchitecturesAllowed=x64compatible
ArchitecturesInstallIn64BitMode=x64compatible
MinVersion=10.0.17763
OutputDir=..\dist
OutputBaseFilename=Workshop-Device-Setup-{#AppVersion}
SetupIconFile=..\assets\app.ico
UninstallDisplayIcon={app}\Workshop-Device.exe
LicenseFile=..\LICENSE
Compression=lzma2
SolidCompression=yes
WizardStyle=modern
DisableProgramGroupPage=yes
CloseApplications=yes
RestartApplications=no

[Languages]
Name: "english"; MessagesFile: "compiler:Default.isl"
Name: "brazilianportuguese"; MessagesFile: "compiler:Languages\BrazilianPortuguese.isl"
Name: "japanese"; MessagesFile: "compiler:Languages\Japanese.isl"

[Tasks]
Name: "desktopicon"; Description: "{cm:CreateDesktopIcon}"; GroupDescription: "{cm:AdditionalIcons}"; Flags: unchecked

[Files]
Source: "{#PackageDir}\*"; DestDir: "{app}"; Flags: ignoreversion recursesubdirs createallsubdirs

[Icons]
Name: "{autoprograms}\Workshop Device"; Filename: "{app}\Workshop-Device.exe"
Name: "{autodesktop}\Workshop Device"; Filename: "{app}\Workshop-Device.exe"; Tasks: desktopicon

[Run]
Filename: "{app}\Workshop-Device.exe"; Description: "{cm:LaunchProgram,Workshop Device}"; Flags: nowait postinstall skipifsilent
