# Windows Host

## Basics

- Look for file extensions
- initial access and lateral movement are the loudest
- understand how PID and PPID relate
- look for 1-2 character .exe   example(a.exe, ab.exe)
- C2s exploits are native in 32 bit
- files should not have read, write and execute
    - should be RW- ro --X
- know where attackers store files
- C:\windows\system32 # exe are not usually stored here

## common malware names
- svchost.exe
- iexplore.exe
- explorer.exe
- lsass.exe
- win.exe
- winlogon.exe

## Common malware locations
- \Temp
- C:\Users\*\Downloads
- \Appdata
  - C:\Users\*\AppData\Roaming\Microsoft\Windows\Recent
- \$Recycle.Bin
- \ProgramData
- \Windows
- \Windows\System32
- \WinSxS
- \System Volume Information
- \Program Files
- \Program Files (x86)
- [Added Directories by APTs]

## Interesting Search terms

#### Scripts
- `.ps1`, `.vbs`, `.py`, `.bat`

#### Windows Binaries
- `.exe`, `.msi`, `.dll`

#### Archives
- `.rar`, `.zip`, `.cab`, `.7z`, `.Eo1`, `.iso`, `.ova`, `.ovf`, `.vmdk`, `.vdk`

- .eval
- .xls
- .doc
- ActiveXObject
- CommandLineTemplate
- ScriptText

## Locations of peristence
  - C:\windows\system32 # exe are not usually stored here

## Types of persistence
- [[Impacket Exec]]
- [[Services]]
- [[WMI]]
- [[autostart]]
- [[DLL Hijacking]]
- [[Drivers]]
- [[Map Share]]
- [[Persistence Mechanisms]]
- [[Powershell Remoting]]
- [[PsExec]]
- [[Remote Desktop]]
- [[Run_Keys]]
- [[SANS_DFPS_FOR508_v4.11_0624]]
- [[Scheduled Tasks]]
- task scheduler
- registry

## Advanced Persistence
- [[Bios Flashing]]
- [[Drivers]]
- [[Local Group Policy]]
- [[MS Office Add-In]]


## EVENT IDS to watch
- 4698 A scheduled task was created 
- 4720 A user account was created 4768 A Kerberos authentication ticket (TGT) was requested 
- 4769 A Kerberos service ticket was requested 
- 5140 A network share object was accessed 
- 7045 A new service was installed in the system.
- 4648 A logon was attempted using explicit credentials 
- 4656 A handle to an object was requested 
- 4658 The handle to an object was closed 
- 4660 An object was deleted 
- 4663 An attempt was made to access an object 
- 4672 Special privileges assigned to new logon 
- 4673 A privileged service was called 4688 A new process has been created 
- 4946 A change has been made to Windows Firewall exception list. A rule was added 
- 5142 A network share object was added 
- 5144 A network share object was deleted 
- 5145 A network share object was checked to see whether the client can be granted desired access 
- 5154 The Windows Filtering Platform has permitted an application or service to listen on a port for incoming connections 
- 5156 The Windows Filtering Platform has allowed a connection 
- 5447 A Windows Filtering Platform filter has been changed 
- 8222 Shadow copy has been created 
- 7036 Service changed
- 7040 Service startup type changed
- 7045 PSExec

4634,4648,4656,4658,4660,4663,4672,4673,4688,4689,4698,4720,4768,4769,4946,5140,5142,5144,5145,5154,5156,5447,8222,7036,7045

## Common false positives
- SCM Event Log Consumer
- BVTFilter
- TSLogonEvents.vbs
- TSLogonFilter
- RAevent.vbs
- RMAssistEventFilter
- KernCap.vbs
- NTEventLogConsumer
- WSCEAA.exe (dell)

### Windows Directories
- C:\Windows\System32\drivers\etc\hosts (dns file)
- C:\Windows\System32\drivers\etc\networks (network config file)
- C:\Windows\System32\config\SAM (usernames and passwords)
- C:\Windows\System32\SECURITY (security logs)
- C:\Windows\System32\SOFTWARE (software logs)
- C:\Windows\System32\SYSTEM (system logs)
- C:\Windows\System32\winevt\ (windows event logs)
- C:\Windows\repair\SAM (Backup of usernames and passwords)


[[pyramid of pain]]

# Analysis

- [[CHeck Filehash]]
- [[Analysis Threat Intel]]
- [[Analysis IP]]
- [[Analysis Filehash]]
- [[Analysis Malware]]


# Resources
---
https://www.youtube.com/watch?v=NdwTeSi70SU
https://youtu.be/7dEfKn70HCI?si=MP-u-n4FMHVgtmWf
https://www.criticalstart.com/windows-security-event-logs-what-to-monitor/
SANS FOR508