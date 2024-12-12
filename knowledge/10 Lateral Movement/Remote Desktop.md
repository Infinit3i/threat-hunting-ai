#rdp #remote_desktop_protocol #rdpclip #tstheme

#### Source Event Logs
- `security.evtx`
    - `4648` - Logon specifying alternate credentials - if NLA enabled on destination
        - Current logged-on User Name
        - Alternate User Name
        - Destination Host Name/IP
        - Process Name
- `Microsoft-Windows-TerminalServices-RDPClient%4Operational.evtx`
    - `1024`
        - Destination Host Name
    - `1102`
        - Destination IP Address

#### Destination Event Logs

- **Security Event Log** – `security.evtx`
    - `4624` Logon Type 10
        - Source IP/Logon User Name
    - `4778/4779`
        - IP Address of Source/Source System Name
        - Logon User Name
- `Microsoft-Windows-RemoteDesktopServices-RdpCoreTS%4Operational.evtx`
    - `131` - Connection Attempts
        - Source IP
    - `98` - Successful Connections
- `Microsoft-Windows-TerminalServices-RemoteConnectionManager%4Operational.evtx`
    - `1149` 
        - Source IP/Logon User Name
            - Blank user name may indicate use of Sticky Keys
- `Microsoft-Windows-TerminalServices-LocalSessionManager%4Operational.evtx`
    - 21, 22, 25
        - Source IP/Logon User Name
    - 41
        - Logon User Name

### Source Registry
- Remote desktop destinations are tracked per-user
    - `NTUSER\Software\Microsoft\Terminal Server Client\Servers`
- [[ShimCache]] – SYSTEM
    - `mstsc.exe` Remote Desktop Client
- [[BAM_DAM]] – SYSTEM – Last Time Executed
    - `mstsc.exe` Remote Desktop Client
- [[AmCache.hve]] - First Time Executed
    - `mstsc.exe`
- UserAssist – `NTUSER.DAT`
    - `mstsc.exe` Remote Desktop Client execution
    - Last Time Executed
    - Number of Times Executed
- RecentApps – `NTUSER.DAT`
    - `mstsc.exe`
    - Remote Desktop Client execution
    - Last Time Executed
    - Number of Times Executed
    - RecentItems subkey tracks connection destinations and times

#### Destination Registry
- [[ShimCache]] - SYSTEM
    - `rdpclip.exe`
    - `tstheme.exe`
- [[AmCache.hve]] - First Time Executed
    - `rdpclip.exe`
    - `tstheme.exe`

### Source File System
- Jumplists - `C:\Users\<Username>\AppData\Roaming\Microsoft\Windows\Recent\AutomaticDestinations\`
    - `{MSTSC-APPID}-automaticDestinations-ms`
    - Tracks remote desktop connection destination and times
- [[Prefetch]] – `C:\Windows\Prefetch\`
    - `mstsc.exe-{hash}.pf`
- [[Bitmap_Cache]] – `C:\Users\<Username>\AppData\Local\Microsoft\TerminalServer Client\Cache`
    - bcache##.bmc
    - cache####.bin
- Default.rdp file –
    - `C:\Users\<Username>\Documents\`

#### Destination File System
- Prefetch – `C:\Windows\Prefetch\`
- `rdpclip.exe-{hash}.pf`
- `tstheme.exe-{hash}.pf`

---

[RDP Authentication vs. Authorization - 13Cubed](https://youtu.be/OlENso8_u7s?si=GGIjhBXsChCb8OvB)

#RDP_NLA
remote desktop access protocol network level authentication
- rdp happens prior to session being established on the system
- before NLA you would have to establish the connection then authenticate

#RDP_successful_logon
event codes
1149 > 4624 type 10(established) OR 7(reconnect) > 21 > 22
- 1149 user authentication succeeded
- 4624 account was successfully logged on
- 21 remote desktop services: session logon succeeded
- 22 remote desktop services: shell start notification received

#MAIN_TAKEAWAY : know when authentication can fail and authorization can succeed