; HelixBench Windows installer (NSIS, per-user, no admin required).
Unicode true
RequestExecutionLevel user

!define PRODUCT_NAME "HelixBench"
!define PRODUCT_VERSION "0.1.0"
!define PRODUCT_PUBLISHER "Shaik Moosa"
!define PRODUCT_WEB_SITE "https://github.com/ShaikMoosa/helixbench"
!define UNINSTALL_REG "Software\Microsoft\Windows\CurrentVersion\Uninstall\HelixBench"

Name "${PRODUCT_NAME}"
OutFile "..\..\dist\HelixBench-Setup-${PRODUCT_VERSION}.exe"
InstallDir "$LOCALAPPDATA\Programs\HelixBench"
InstallDirRegKey HKCU "${UNINSTALL_REG}" "InstallLocation"
Icon "..\..\apps\desktop_gpui\assets\helixbench.ico"
UninstallIcon "..\..\apps\desktop_gpui\assets\helixbench.ico"
ShowInstDetails show
ShowUnInstDetails show

VIProductVersion "0.1.0.0"
VIAddVersionKey "ProductName" "${PRODUCT_NAME}"
VIAddVersionKey "ProductVersion" "${PRODUCT_VERSION}"
VIAddVersionKey "FileVersion" "${PRODUCT_VERSION}"
VIAddVersionKey "CompanyName" "${PRODUCT_PUBLISHER}"
VIAddVersionKey "FileDescription" "${PRODUCT_NAME} Setup"
VIAddVersionKey "LegalCopyright" "MIT"

!include "MUI2.nsh"
!define MUI_ABORTWARNING
!define MUI_ICON "..\..\apps\desktop_gpui\assets\helixbench.ico"
!define MUI_UNICON "..\..\apps\desktop_gpui\assets\helixbench.ico"
!define MUI_FINISHPAGE_RUN "$INSTDIR\helixbench_gpui.exe"
!define MUI_FINISHPAGE_RUN_TEXT "Launch HelixBench"

!insertmacro MUI_PAGE_WELCOME
!insertmacro MUI_PAGE_DIRECTORY
!insertmacro MUI_PAGE_COMPONENTS
!insertmacro MUI_PAGE_INSTFILES
!insertmacro MUI_PAGE_FINISH
!insertmacro MUI_UNPAGE_CONFIRM
!insertmacro MUI_UNPAGE_INSTFILES
!insertmacro MUI_LANGUAGE "English"

Section "HelixBench (required)" SecCore
  SectionIn RO
  SetOutPath "$INSTDIR"
  File "..\..\target\release\helixbench_gpui.exe"
  File "..\..\apps\desktop_gpui\assets\helixbench.ico"

  ; Start Menu shortcut
  CreateShortCut "$SMPROGRAMS\HelixBench.lnk" "$INSTDIR\helixbench_gpui.exe" "" "$INSTDIR\helixbench.ico" 0

  ; Uninstaller + Add/Remove Programs entry (per-user)
  WriteUninstaller "$INSTDIR\uninstall.exe"
  WriteRegStr HKCU "${UNINSTALL_REG}" "DisplayName" "${PRODUCT_NAME}"
  WriteRegStr HKCU "${UNINSTALL_REG}" "DisplayVersion" "${PRODUCT_VERSION}"
  WriteRegStr HKCU "${UNINSTALL_REG}" "Publisher" "${PRODUCT_PUBLISHER}"
  WriteRegStr HKCU "${UNINSTALL_REG}" "URLInfoAbout" "${PRODUCT_WEB_SITE}"
  WriteRegStr HKCU "${UNINSTALL_REG}" "DisplayIcon" "$INSTDIR\helixbench.ico"
  WriteRegStr HKCU "${UNINSTALL_REG}" "InstallLocation" "$INSTDIR"
  WriteRegStr HKCU "${UNINSTALL_REG}" "UninstallString" "$\"$INSTDIR\uninstall.exe$\""
  WriteRegStr HKCU "${UNINSTALL_REG}" "QuietUninstallString" "$\"$INSTDIR\uninstall.exe$\" /S"
  WriteRegDWORD HKCU "${UNINSTALL_REG}" "NoModify" 1
  WriteRegDWORD HKCU "${UNINSTALL_REG}" "NoRepair" 1
SectionEnd

Section "Desktop shortcut" SecDesktop
  CreateShortCut "$DESKTOP\HelixBench.lnk" "$INSTDIR\helixbench_gpui.exe" "" "$INSTDIR\helixbench.ico" 0
SectionEnd

!insertmacro MUI_FUNCTION_DESCRIPTION_BEGIN
  !insertmacro MUI_DESCRIPTION_TEXT ${SecCore} "The HelixBench application and its icon."
  !insertmacro MUI_DESCRIPTION_TEXT ${SecDesktop} "Create a desktop shortcut."
!insertmacro MUI_FUNCTION_DESCRIPTION_END

Section "Uninstall"
  Delete "$SMPROGRAMS\HelixBench.lnk"
  Delete "$DESKTOP\HelixBench.lnk"
  Delete "$INSTDIR\helixbench_gpui.exe"
  Delete "$INSTDIR\helixbench.ico"
  Delete "$INSTDIR\uninstall.exe"
  RMDir "$INSTDIR"
  DeleteRegKey HKCU "${UNINSTALL_REG}"
SectionEnd