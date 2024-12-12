# Defensive  Methods

[[Knowing Your Network]]

[[Baseline process]] - Setting up and knowing your infastructure

## Incident Response Investigation Funnel [^1]

### 1. Time of Incident
- **Sources:**
  - SIEM/IDS/AV alert
  - 3rd Party Notification

### 2. Network Activity
- **Indicators:**
  - Malicious URLs accessed
  - DNS requests for bad domains

### 3. Process Activity
- **Indicators:**
  - Running process related to incident
  - DLL injection detected

### 4. Name of a File
- **Indicators:**
  - File name of interest (e.g., `p.exe`, `r1.exe`)
  - File type of interest (e.g., `.rar`, `.py`, `.ps1`)

### 5. User Account Activity
- **Indicators:**
  - Identify suspicious user account activity

### 6. Other Activity
- **Indicators:**
  - Lateral Movement (Event Logs + File Copy & Execution)
  - Anti-Forensics (Wiper download, wiper execution)


- [[Hunting Method(win)]]
- [[Hunting Method(nix)]]
- [[IR Method]]
- [[Malware Analysis Method]]


### References
[^1]: FOR508 Book5 27

