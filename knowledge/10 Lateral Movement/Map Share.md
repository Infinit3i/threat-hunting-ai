#map #share

### Source Event Logs
---
- `security.evtx`
    - `4648` - Logon specifying alternate credentials
        - Current logged-on User Name
        - Alternate User Name
        - Destination Host Name/IP
        - Process Name 
- `Microsoft-Windows-SmbClient\Security.evtx`
    - `31001` – Failed logon todestination
        - Destination Host Name
        - User Name for failed logon
        - Reason code for failed destination logon (e.g., bad password)

### Destination Event Logs
---
Security Event Log – security.evtx
- `4624`
    - Logon Type 3
    - Source IP/Logon User Name
- `4672`
    - Logon User Name
    - Logon by user with administrative rights
    - Requirement for accessing default shares such as **C$** and **ADMIN$**
- `4776` - NTLM if authenticating to Local System
    - Source Host Name/Logon User Name
- `4768` - TGT Granted
    - Source Host Name/Logon User Name
    - Available only on domain controller
- `4769` - Service Ticket Granted if
authenticating to Domain Controller
    - Destination Host Name/Logon User Name
    - Source IP
    - Available only on domain controller
- `5140`
    - Share Access
- `5145`
    - Auditing of shared files – **NOISY**!

### Source Registry
---
MountPoints2 - Remotely mapped shares
`NTUSER\Software\Microsoft\Windows\CurrentVersion\Explorer\MountPoints2`
Shellbags - USRCLASS.DAT
Remote folders accessed inside an interactive session via
Explorer by attackers
- [[ShimCache]] – SYSTEM
    - `net.exe`
    - `net1.exe`
- [BAM_DAM] – NTUSER.DAT – Last Time Executed
    - `net.exe`
    - `net1.exe`
- [[AmCache.hve]] - First time executed
    - `net.exe`
    - `net1.exe`

### Destination Registry
- N/A

### Source File System
---
- Prefetch – `C:\Windows\Prefetch\`
    - `net.exe-{hash}.pf`
    - `net1.exe-{hash}.pf`
- User profile Artifacts
    - Review shortcut files and jumplists for remote files accessed by attackers, if they had interactive access (RDP)

### Destination File System
---
- File Creation
    - Attacker's files (malware) copied to destination system
    - Look for Modified Time before Creation Time
    - Creation Time is time of file copy
- User Access Logging (Servers only)
    - `C:\Windows\System32\LogFiles\Sum`
    - User Name
    - Source IP Address
    - First and Last Access Time
