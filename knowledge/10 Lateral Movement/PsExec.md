`psexec.exe \\host -accepteula -d -c c:\temp\evil.exe`

Host process name: `psexec.exe`
client process name: `psexesvc.exe`

# Command ran
`psexec.py domain/username:password@[hostname | IP] command`
- Can specify a command to run, or leave blank for shell
- PSEXEC like functionality example using RemComSvc
- Creates and subsequently deletes a Windows Service with a random 4-character mixed-case alpha name referencing an 8-character mixed-case alpha .exe file in %systemroot%
- Detected and blocked by Windows Defender by default

### Source Event Logs
---
- `security.evtx`
    - `4648` - Logon specifying alternate credentials
        - Current logged-on User Name
        - Alternate User Name
        - Destination Host Name/IP
        - Process Name

### Destination Event Logs
---
`security.evtx`
- `4648` Logon specifying alternate credentials
    - Connecting User Name
    - Process Name
- `4624` Logon Type 3 (and Type 2 if “-u” Alternate Credentials are used)
    - Source IP/Logon User Name
- `4672`
    - Logon User Name
    - Logon by a user with administrative rights
    - Requirement for access default shares such as **C$** and **ADMIN$**
- `5140` – Share Access
    - **ADMIN$** share used by PsExec 

# Windows Event Log Residue:
- Event ID `4776` in Security on target (for user specified in command)
- Event ID `4672` in Security on target (for user specified in command)
- Event ID `4624` Type 3 in Security on target (for user specified in command)
- Event ID `7045` in System on target (service installation: 4-character mixed-case alpha name referencing an 8-character mixed-case alpha .exe file):
    - %systemroot%\xxxxxxxx.exe
- Event ID `7036` in System on target
- Event ID `7036` in System on target
- [IF ENABLED] Event ID 4688 in Security on target:
    - services.exe → C:\Windows\xxxxxxxx.exe
- Event ID `4776` in Security on target (for user specified in command)
- Event ID `4672` in Security on target (for user specified in command)
- Event ID `4624` Type 3 in Security on target (for user specified in command)
- Event ID `4776` in Security on target (for user specified in command)
- Event ID `4672` in Security on target (for user specified in command)
- Event ID `4624` Type 3 in Security on target (for user specified in command)
- Event ID `4776` in Security on target (for user specified in command)
- Event ID `4672` in Security on target (for user specified in command)
- Event ID `4624` Type 3 in Security on target (for user specified in command)
- [IF ENABLED] Event ID 4688 in Security on target:
    - C:\Windows\xxxxxxxx.exe → command
- [IF ENABLED] Event ID 4688 in Security on target:
    - cmd.exe → conhost.exe 0xffffffff -ForceV1
- ... numerous other `4624`,`4634`,`4672` events

---

### Source Registry
---
- NTUSER.DAT
    - Software\SysInternals\PsExec\EulaAccepted
 - [[ShimCache]] – SYSTEM
    - psexec.exe
- [[BAM_DAM]] – SYSTEM – Last Time Executed
    - psexec.exe
- [[AmCache.hve]] – First Time Executed
    - psexec.exe

### Destination Registry
---
- New service creation configured in `SYSTEM\CurrentControlSet\Services\PSEXESVC`
    - “ -r” option can allow attacker to rename service
- [[ShimCache]] - SYSTEM `psexesvc.exe`
- [[AmCache.hve]]
    - First Time Executed `psexesvc.exe`

---

### Source File System
---
- Prefetch – C:\Windows\Prefetch\
    - psexec.exe-{hash}.pf
    - Possible references to other files accessed by psexec.exe, such as executables copied to target system with the “-c” option
- File Creation
    - psexec.exe file downloaded and created on local host as the file is not native to Windows

`system.evtx`
- `7045`
    - Service Install

### Destination File System
---
- Prefetch – `C:\Windows\Prefetch\`
    - `psexesvc.exe-{hash}.pf`
    - `evil.exe-{hash}.pf`
- File Creation
    - User profile directory structure created unless "-e" option used
    - psexesvc.exe will be placed in **ADMIN$** (\Windows) by default, as well as other executables (evil.exe) pushed by PsExec
- User Access Logging (Servers only)
    - `C:\Windows\System32\LogFiles\Sum`
        - User Name
        - Source IP Address
        - First and Last Access Time

#### Resources
[13cubed Impacket Exec](https://www.youtube.com/watch?v=UMogme3rDRA&t=1752s&ab_channel=13Cubed)
[Sans FOR508 Poster](https://www.sans.org/posters/hunt-evil/)