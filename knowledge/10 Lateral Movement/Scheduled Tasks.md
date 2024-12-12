#scheduled_tasks #schedule

`at \\host 13:00 "c:\temp\evil.exe"`

`schtasks /CREATE /TN taskname /TR c:\temp\evil.exe /SC once /RU “SYSTEM” /ST 13:00 /S host /U username`

### Source Event Logs
#scheduled_tasks #schedule #at #schtasks
- `security.evtx`
    - `4648` - Logon specifying alternate credentials
        - Current logged-on User Name
        - Alternate User Name
        - Destination Host Name/IP
        - Process Name

### Destination Event Logs
- `security.evtx`
    - `4624` Logon Type 3
        - Source IP/Logon User Name
    - `4672`
        - Logon User Name
        - Logon by a user with administrative rights
        - Requirement for accessing default shares such as **C$** and **ADMIN$**
    - `4698` - Scheduled task created
    - `4702` - Scheduled task updated
    - `4699` - Scheduled task deleted
    - `4700/4701` - Scheduled task enabled/disabled

- `Microsoft-Windows-TaskScheduler%4Operational.evtx`
    - `106` - Scheduled task created
    - `140` - Scheduled task updated
    - `141` - Scheduled task deleted
    - `200/201` - Scheduled task executed/completed

### Source Registry
- [[ShimCache]] - SYSTEM
    - at.exe
    - schtasks.exe
- [[BAM|DAM]] - SYSTEM - Last Time Executed
    - at.exe
    - schtasks.exe
- [[AmCache.hve]] - First Time Executed
    - at.exe
    - schtasks.exe

### Destination Registry
- SOFTWARE
    - `Microsoft\Windows NT\CurrentVersion\Schedule\TaskCache\Tasks`
    - `Microsoft\Windows NT\CurrentVersion\Schedule\TaskCache\Tree\`
- [[ShimCache]] – SYSTEM
    - evil.exe
- [[AmCache.hve]] - First Time Executed
    - evil.exe

### Source File System
---
- [[Prefetch]] - C:\Windows\Prefetch\
    - at.exe-{hash}.pf
    - schtasks.exe-{hash}.pf

### Destination File System
- File Creation
    - evil.exe
- Job files created in
    - `C:\Windows\Tasks`
- XML task files created in
    - `C:\Windows\System32\Tasks`
    - `C:\Windows\SysWOW64\Tasks`
    - Author tag can identify:
        - Source system name
        - Creator username
- [[Prefetch]] – `C:\Windows\Prefetch\`
    - evil.exe-{hash}.pf