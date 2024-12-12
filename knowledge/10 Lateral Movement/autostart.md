# Autostart
Sans FOR508.1 p70

AutoStart Persistence Locations are numerous in Windows, often referred to as AutoStart Extension Points (ASEPs). These locations help identify programs that start automatically at system boot or user logon. Some common ASEP locations include:

- `NTUSER.DAT\Software\Microsoft\Windows\CurrentVersion\Run`
- `NTUSER.DAT\Software\Microsoft\Windows\CurrentVersion\RunOnce`
- `SOFTWARE\Microsoft\Windows\CurrentVersion\RunOnce`
- `SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\Explorer\Run`
- `SOFTWARE\Microsoft\Windows\CurrentVersion\Run`
- `SOFTWARE\Microsoft\Windows NT\CurrentVersion\Winlogon\Userinit`
- `%AppData%\Roaming\Microsoft\Windows\Start Menu\Programs\Startup`

Investigative notes suggest these locations are excellent starting points to check for malicious activity. These are only a fraction of possible locations, and analyzing AutoStart data across many systems might help identify compromised systems. ASEPs are key to why Windows can be vulnerable, and blogs show well over 50 ASEP locations that can be used by malicious files to maintain persistence after a reboot.

The most common ASEPs are the “Run” Registry keys:
- `NTUSER.DAT\Software\Microsoft\Windows\CurrentVersion\Run`
- `NTUSER.DAT\Software\Microsoft\Windows\CurrentVersion\RunOnce`
- `Software\Microsoft\Windows\CurrentVersion\RunOnce`
- `Software\Microsoft\Windows\CurrentVersion\Policies\Explorer\Run`
- `Software\Microsoft\Windows\CurrentVersion\Run`

Items in these keys are executed when a user logs on, unlike other ASEPs that act at boot. Multiple "run" keys exist in both the NTUSER.DAT and SOFTWARE hives. Another, less common but equally dangerous key is `SOFTWARE\Microsoft\Windows NT\CurrentVersion\Winlogon\Userinit`. This key typically contains a reference to `userinit.exe`, which by default executes Userinit.exe and launches Explorer.exe. However, it can be modified to include malicious binaries, such as `C:\Windows\system32\userinit.exe,C:\Temp\winsvchost.exe`, which would run at boot.

Finally, `%AppData%\Roaming\Microsoft\Windows\Start Menu\Programs\Startup` allows for persistence by placing shortcuts in this folder, which automatically execute the associated binaries when a user logs on. Malware has recently gravitated back to this old attack vector. Although these locations are very common for ASEPs, many more exist. Tools like Registry Explorer and RegRipper can retrieve additional ASEPs from Registry hives, and analysis of data across systems may reveal outliers leading to compromised systems.