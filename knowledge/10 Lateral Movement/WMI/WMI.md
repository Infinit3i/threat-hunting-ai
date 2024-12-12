## Overview:

Windows Management Instrumentation (WMI) is a framework that allows users to manage and access resources on Windows-based operating systems

**source** machine process name: `wmic.exe`

**destination** machine process name: `wmiprvse.exe`

## Example

`wmic /node:host process call create "C:\temp\evil.exe"`

`Invoke-WmiMethod –Computer host –Class Win32_Process –Name create –Argument “c:\temp\evil.exe"`

# Source Artifacts

### Source Event Logs
---
`security.evtx`
- 4648 – Logon specifying alternate credentials
    - Current logged-on User Name
    - Alternate User Name
    - Destination Host Name/IP
    - Process Name

### Source Registry
---
- [[ShimCache]] – SYSTEM
    - wmic.exe
- [[BAM_DAM]] – SYSTEM – Last Time Executed
    -  wmic.exe
- [[AmCache.hve]] - First Time Executed
    - wmic.exe

### Source File System
---

[[Prefetch]] – C:\Windows\Prefetch\
wmic.exe-{hash}.pf



# Destination Artifacts

### Destination Event Logs
---
`security.evtx`
- 4624 Logon Type 3
    - Source IP/Logon User Name
- 4672
    - Logon User Name
    - Logon by an a user with administrative rights

`Microsoft-Windows-WMI-Activity/Operational.evtx`
- 5857
    - Indicates time of wmiprvse execution and path to provider DLL – attackers sometimes install malicious WMI provider DLLs
- 5860, 5861
    - Registration of Temporary (5860) and Permanent (5861) Event Consumers.
    - Typically used for persistence, but can be used for remote execution.

### Destination Registry
---

- [[ShimCache]] – SYSTEM
    - scrcons.exe
    - mofcomp.exe
    - wmiprvse.exe
    - evil.exe
- [[AmCache.hve]] - First Time Executed
    - scrcons.exe
    - mofcomp.exe
    - wmiprvse.exe
    - evil.exe

### Destination File System
---

- File Creation
    - evil.exe
    - evil.mof - .mof files can be used to manage the WMI Repository
- [[Prefetch]] – C:\Windows\Prefetch\
    - scrcons.exe-{hash}.pf
    - mofcomp.exe-{hash}.pf
    - wmiprvse.exe-{hash}.pf
    - evil.exe-{hash}.pf
- Unauthorized changes to the WMI Repository in `C:\Windows\System32\wbem\Repository`