## Overview:

PowerShell remoting is a feature that allows users to run PowerShell commands on remote computers. It uses Windows Remote Management (WinRM), which is Microsoft's implementation of the Web Services for Management (WS-Management) protocol.

**Source** machine process name: `powershell.exe`

**Destination** machine process name: `wsmprovhost.exe`

## Example

`Enter-PSSession –ComputerName host`

`Invoke-Command –ComputerName host –ScriptBlock {Start-Process c:\temp\evil.exe}`

# Source Artifacts

### Source Event Logs
---
- `security.evtx`
    - `4648` - Logon specifying alternate credentials
        - Current logged-on User Name
        - Alternate User Name
        - Destination Host Name/IP
        - Process Name'

- `Microsoft-Windows-WinRM/Operational.evtx`
    - `161` - Remote Authentication Error
    - `6` - WSMan Session initialize
        - Session created
        - Destination Host Name or IP
        - Current logged-on User Name
    - `8` , `15`, `16`, `33` - WSMan Session deinitialization
        - Closing of WSMan session
        - Current logged-on User Name

`Microsoft-Windows-PowerShell/Operational.evtx`
- `40961`, `40962`
    - Records the local initiation of powershell.exe and associated user account
- `8193` & `8194`
    - Session created
- `8197` - Connect
    - Session closed

### Source Registry
---
- [[ShimCache]] – SYSTEM
    - powershell.exe
- [[BAM_DAM]] – SYSTEM – Last Time Executed
    - powershell.exe
- [[AmCache.hve]] – First Time Executed
    - powershell.exe


### Source File System
---
- [[Prefetch]] – C:\Windows\Prefetch\
    - powershell.exe-{hash}.pf
    - PowerShell scripts (.ps1 files) that run within 10 seconds of powershell.exe launching will be tracked in powershell.exe prefetch file
- Command history
    - `C:\Users\<Username>\AppData\Roaming\Microsoft\Windows\PowerShell\PSReadline\ConsoleHost_history.txt`
        - With PS v5+, a history file with previous 4096 commands is maintained per user

# Destination Artifacts

### Destination Event Logs
---
- `security.evtx`
    - `4624` – Logon Type 3
        - Source IP/Logon User Name
    - `4672`
        - Logon User Name
        - Logon by an a user with administrative rights

-`Microsoft-Windows-PowerShell%4Operational.evtx`
    - `4103`, `4104` – Script Block logging
        - Logs suspicious scripts by default in PS v5
        - Logs all scripts if configured
    - `53504` - Records the authenticating user

`Windows PowerShell.evtx`
- `400/403` "ServerRemoteHost" indicates start/end of Remoting session
- `800` Includes partial script code

`Microsoft-Windows-WinRM/Operational.evtx`
- `91` – Session creation
- `142` – WSMan Operation Failure
- `169` – Records the authenticating user


### Destination Registry
---
[[ShimCache]] – SYSTEM
- wsmprovhost.exe
- evil.exe
- SOFTWARE
    - `Microsoft\PowerShell\1\ShellIds\Microsoft.PowerShell\ExecutionPolicy`
        - Attacker may change execution policy to a less restrictive setting, such as "bypass"

[[AmCache.hve]] – First Time Executed
- wsmprovhost.exe
- evil.exe


### Destination File System
---
- File Creation
    - evil.exe
    - With Enter-PSSession, a user profile directory may be created
- [[Prefetch]] – C:\Windows\Prefetch\
    - evil.exe-{hash].pf
    - wsmprovhost.exe-{hash].pf
