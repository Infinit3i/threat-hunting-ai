# Hunting Notes: DLL Hijacking

DLL hijacking is excellent for persistence but is also commonly used to evade host-based security and to achieve privilege escalation.

- **File system analysis:** Look for new or unsigned .exe/.dll files in unusual places.

## Memory Analysis: Find system processes or DLLs loaded from wrong locations

| Timestamp           | Impact | Method | File Name/Hash/File Size                |
|---------------------|--------|--------|-----------------------------------------|
| 2021-02-18 03:42:31 | -      | mach Meta | c:/ProgramData/mcoemcpy.exe (77824)     |
| 2021-02-18 03:42:31 | -      | mach Meta | c:/ProgramData/McUtil.dll (131072)      |

- **Command Line:** `C:\Programdata\ncoenchy.exe 0x4`

DLL hijacking is often paired with other tradecraft like code injection and command-and-control network beaconing that can also lead to detection.

## SANS DFIR

DLL hijacking can be a very stealthy attacker technique but is often easily defeated with thorough forensic analysis. Nearly all DLL hijacks require placing a new DLL and/or executable onto the file system. This presents a useful detection point since newly created DLLs and executables are rare on most systems. Paying attention to newly created files around a time of interest is a classic forensic timelining technique that we will see later in the class. Memory forensics is another technique well suited to finding code running from unusual locations since all legitimately loaded code must come from disk. By looking for processes that should not be running, or those loading DLLs from strange locations, an analyst could easily find evidence of DLL hijacking in memory. Also note that hijacked DLLs tend to be the more obscure ones because the most common DLLs are already loaded in memory, preempting the loading of any evil versions with duplicate names (Windows first checks if a DLL is already loaded in memory before looking on disk). Added to this is code loaded through hijacking almost always performs other actions likely to draw the eye of an investigator. These could include reaching out over the network, creating named pipes, or injecting code into other processes. We will have many ways throughout the course to detect anomalous actions and any of them could lead back to a DLL in a strange location, and ultimately help you discover a DLL hijack attack.

The examples on this slide show evidence of a relative path DLL hijack attack commonly tied to the Ocean Lotus/APT32 threat actor. The executable `mcoemepy.exe` is legitimate, digitally signed software from McAfee. It has been copied to an unusual location (ProgramData folder) along with a malicious DLL, `McUtil.dll`. Upon execution, `mcoemepy.exe` also loads `McUtil.dll`. A keen analyst might wonder why these newly created files were present in the root of the ProgramData folder, a location not typically known to hold executable files. If some of the tool output on this slide is unfamiliar to you, don't worry, we will be covering them in depth in later sections. Another interesting detection has been proposed by Matt Green of the Velociraptor project. In many hijacks, the malicious DLL still needs to proxy legitimate function requests to the original DLL. Creating forwarded functions within the evil DLL is an easy way to accomplish this. While forwarded functions can be legitimate, it is rare to see two DLLs with the same name (but different locations) forwarding functions to each other, especially outside of folders like WinSxS. Velociraptor has VQL designed to look for this pattern.

[1] Detecting DLL Hijacking With Velociraptor; [6](https://for508.com/tsgpy)
(1) https://blog.blueinktech.com/blog/how-does-the-fmcsa-revoke-elds-and-is-your-eld.... https://blog.blueinktech.com/blog/how-does-the-fmcsa-revoke-elds-and-is-your-eld-provider-at-risk.
(2) https://chipnetics.com/projects/fw/snesparallel. https://chipnetics.com/projects/fw/snesparallel/.
(3) https://forum.level1techs.com/t/asrock-rack-introduces-the-paul-an-ipmi-pcie-card.... https://forum.level1techs.com/t/asrock-rack-introduces-the-paul-an-ipmi-pcie-card/162977?page=7.
(4) https://www.chegg.com/homework-help/questions-and-answers/project-description-task.... https://www.chegg.com/homework-help/questions-and-answers/project-description-task-project-make-three-following-improvements-realistic-grocery-store-q103184148.
(5) https://www.twz.com/tic-tac-uap-incident-included-in-72-newly-released-range-incursion.... https://www.twz.com/tic-tac-uap-incident-included-in-72-newly-released-range-incursion-reports.
(6) undefined. https://for508.com/tsgpy.