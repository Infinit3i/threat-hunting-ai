#services #sc #ac

## Examples 

`sc \\host create servicename binpath="c:\temp\evil.exe"`

`sc \\host start servicename`

### Source Event Logs
- N/A

### Destination Event Logs
---
- `security.evtx`
    - `4624` Logon Type 3
        - Source IP/Logon User Name
    - `4697`
        - Security records service install, if enabled
        - enabled by non-default security events such as ID 4697 are particularly useful if only the Security logs are forwarded to a centralized log server

- `system.evtx`
    - `7034` - Service crashed unexpectedly
    - `7035` - Service sent a Start/Stop control
    - `7036` - Service started or stopped
    - `7040` - Start type changed (Boot | On Request | Disabled)
    - `7045` - A service was installed on the system

### Source Registry
---
- [[ShimCache]] - SYSTEM
    - `sc.exe`
- [[BAM_DAM]] - SYSTEM - Last Time Executed
    - `sc.exe`
- [[AmCache.hve]]
    - `sc.exe`

### Destination Registry
---
- SYSTEM
    - `\CurrentControlSet\Services\`
    - New service creation
- [[ShimCache]] - SYSTEM
    - `evil.exe`
- [[ShimCache]] records
    - `evil.exe`
    - Shimcache records existence of malicious service executable, unless implemented as a service DLL
- [[AmCache.hve]] - First Time Executed
    - evil.exe

### Source File System
---
- Prefetch - `C:\Windows\Prefetch\`
    - `sc.exe-{hash}.pf`

### Destination File System
---
- File Creation
    - `evil.exe` or `evil.dll` malicious service executable or service DLL
- Prefetch – `C:\Windows\Prefetch\`
    - `evil.exe` -{hash}.pf

