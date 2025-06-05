[Setup]
AppName=Server Version Checker
AppVersion=0.2.0
DefaultDirName={pf}\svcheck
DefaultGroupName=svcheck
UninstallDisplayIcon={app}\svcheck.exe
OutputDir=.
OutputBaseFilename=svcheck-installer
Compression=lzma
SolidCompression=yes

[Files]
Source: "C:\proyect\svcheck\win\svcheck.exe"; DestDir: "{app}"; Flags: ignoreversion
Source: "C:\proyect\svcheck\win\README.txt"; DestDir: "{app}"; Flags: ignoreversion
Source: "C:\proyect\svcheck\win\config.toml"; DestDir: "{app}"; Flags: ignoreversion
Source: "C:\proyect\svcheck\win\README-POST-INSTALL.txt"; DestDir: "{app}"; Flags: ignoreversion

[Icons]
Name: "{group}\svcheck"; Filename: "{app}\svcheck.exe"
Name: "{commondesktop}\svcheck"; Filename: "{app}\svcheck.exe"; Tasks: desktopicon

[Tasks]
Name: "desktopicon"; Description: "Crear icono en el escritorio"; GroupDescription: "Iconos adicionales:"

[Run]
Filename: "{cmd}"; Parameters: "/C setx PATH ""{app};%PATH%"""; Flags: runhidden

[Code]
// Esto refresca el PATH para que esté disponible sin reiniciar sesión
function InitializeSetup(): Boolean;
begin
  MsgBox('Al finalizar la instalación, abre una nueva terminal y escribe: svcheck dominio.com, modifica la informacio el archivo config.toml', mbInformation, MB_OK);
  Result := True;
end;


