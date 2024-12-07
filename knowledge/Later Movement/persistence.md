#Autostart_Extensibility_Points #ASEP

[Persistence Mechanisms 13Cubed](https://youtu.be/ImGaqVHAbCk?si=EprOd-nWTgAP_r3r)

[Common Malware Persistence mechanisms](https://www.infosecinstitute.com/resources/malware-analysis/common-malware-persistence-mechanisms/)

[[Run_Keys]]

## BootExecute Key 
- launches before subsystem

`HKEY_LOCAL_MACHINESYSTEM\ControlSet002\Control\Session`

## WinLogon Process Keys

### Userinit Key
- launch login scripts

`HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\Windows NT\CurrentVersion\Winlogon`

### Notify Key
- description: HANDLES (Ctrl+Alt+Del)

`HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\Windows NT\CurrentVersion\Winlogon\Notify`

### Explorer.exe Key
- description: Points to explorer.exe

`HKEY_LOCAL_MACHINESOFTWAREMicrosoftWindows NTCurrentVersionWinlogonShell`

## Startup Key
- description

`HKEY_CURRENT_USER\Software\Microsoft\Windows\CurrentVersion\Explorer\User Shell\Folders`
`HKEY_CURRENT_USER\Software\Microsoft\Windows\CurrentVersion\Explorer\Shell Folders`
`HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\Shell Folders`
`HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\User Shell Folders`

## Services

description: ability for services to boot at start time

`HKEY_LOCAL_MACHINESYSTEMCurrentControlSetservices`

remote backgrounder services

`HKEY_LOCAL_MACHINESoftwareMicrosoftWindowsCurrentVersionRunServicesOnce`

`HKEY_LOCAL_MACHINESoftwareMicrosoftWindowsCurrentVersionRunServices`

## Browser Helper Objects

description: 

`HKEY_LOCAL_MACHINESOFTWAREMicrosoftWindowsCurrentVersionExplorerBrowser Helper Objects`

## AppInit_DLLs

`HKEY_LOCAL_MACHINESOFTWAREMicrosoftWindows NTCurrentVersionWindowsAppInit_DLLs`



[Persistence using global flags](https://oddvar.moe/2018/04/10/persistence-using-globalflags-in-image-file-execution-options-hidden-from-autoruns-exe/)
```
reg add "HKLM\SOFTWARE\Microsoft\Windows NT\CurrentVersion\Image File Execution Options\notepad.exe" /v GlobalFlag /t REG_DWORD /d 512
reg add "HKLM\SOFTWARE\Microsoft\Windows NT\CurrentVersion\SilentProcessExit\notepad.exe" /v ReportingMode /t REG_DWORD /d 1
reg add "HKLM\SOFTWARE\Microsoft\Windows NT\CurrentVersion\SilentProcessExit\notepad.exe" /v MonitorProcess /d "C:\temp\evil.exe"
```