# DCO TTPs

## AI Cautions
- Incorrect results, information disclosure, privacy concerns, AI hallucination.

## Application Allow List
- Prevents the execution of unauthorized code.

## Audit Permissions and Policies
- Ensuring that the principle of least privilege (POLP) is sufficiently employed falls under what defense recommendation category?

## Business Partners
- They decide when to put a system back into production.

## BPF Type Qualifier
- Host.

## Content Security Policy (CSP)
- Protects against XSS by configuring the web server to declare where linked resources can be loaded from in the requested page by the browser.

## Cloud
- **Listable cloud bucket** - Files can be both enumerated and downloaded.

## DCs
- Need inbound SMB connections.

## Disabling LLMNR
- Defense against hijacking attacks.

## Event IDs (4624, 4634, 4672, 4732, 4648, 4688, 4697, 4768)
- Used to monitor unauthorized access attempts.

## Firewall Alert
- **Determine if IP is malicious** - First action should be identifying if the IP is malicious.

## Incident Response
- **First action** - Verifying whether an incident occurred.

## Golden Ticket Attack
- Resolve by changing the `krbtgt` account password twice.

## Hayabusa
- Reads Windows event logs.

## Hashes
- Dictionary attacks are less effective for attackers due to hashed value protections.

## HTTP Proxy Log
- Used to identify cloud storage use in your organization.

## Investigating Malware
- Monitor the environment and examine code.

## LANMAN
- **Empty LANMAN hash**: `aad3b43b435b51404eeaad3b435b52404ee`.
- **After disabling LANMAN hash storage** - The LANMAN hash disappears after the user changes their password.

## Windows Server 2003
- **Security recommendation** - Remove LANMAN hashing mechanism from the customer's environment.

## Memory Analysis
- Important because information may reside in RAM that cannot be found on disk.

## Password Security
- **Multi-factor authentication (MFA)** - Best mitigation against password-cracking attacks targeting Windows systems.
- **Password length** - Most valuable tool to force passphrase use when selecting passwords.

## SQL Injection
- **Parameterized queries** - Effective defense against SQL injection attacks.

## SID
- **Rid: [1f4] = 500** - Indicates an administrator.

## SMB
- **Valid user credentials** - Required to create a list of domain users.

## Statistical Anomaly Analysis
- Detects C2 and exfiltration techniques that traditional IDS often miss.

## SSRF and IMDS Attacks
- **Defense** - Require IMDSv2 for AWS.

## MITRE ATT&CK Framework
- Based on observations from real-world attacks.

## Sigma
- A signature format for describing threats.

## SQUID
- Used for proxying web traffic.

## PICERL Incident Response Model
- **Biggest issue** - Uses a linear approach without revisiting steps.

### Incident Response Phases
- **Preparation** - Strategic threat intelligence capability to monitor attack trends.
- **Containment Phase** - Applying filters to network devices.
- **Eradication Phase** - Restore systems from trusted backups.

## Tcpdump
- Example: `tcpdump -i eth0 -w capture.pcap` - Saves a packet capture in `.pcap` format.

## UDP
- It is connection-less.

## WAF
- Short-term mitigation against command injection attacks while vulnerabilities are resolved.

## Web Proxy
- **Scenario**: An employee opens a malicious attachment, the Meterpreter payload connects to TCP port 443 of an attacker-controlled IP. The firewall permits outbound connections to any IP on TCP port 443.
  - **Protection**: Web proxy filtering and outbound connection controls.

## Windows Server 2016
- Minimum server version that supports Microsoft SMB version 3.1.1.

## `document.cookie`
- Used to retrieve a cookie that a victim has stored in their browser.

## Network Login Account Audit
- **Why logging in interactively with a list of bad passwords is a bad idea**: High number of failed attempts could trigger an account lockout.

## Rootkit Eradication
- Most cost-effective solution: Reformat, reinstall, and patch the system from the original media.

## IMDS Credential Disclosure
- Affects multiple cloud providers including AWS (IMDSv1), DigitalOcean Droplets, and Alibaba Cloud.

## Cloud Providers and IMDS Mitigation
- Special headers for IMDS (e.g., Microsoft Azure, Google Cloud Platform, AWS IMDSv2) mitigate exploitation through SSRF.

## Malware Analysis - Snapshot
- Provides a high-level summary of changes that occurred after a malware sample was executed.

## Remediation
- **Weak password used**: Investigate why a weak password was allowed in the first place.
- **Root cause analysis** - Identify the vector and methods of attack to build a post-incident monitoring service.

## Goal of Remediation in Incident Response
- Correction of the root cause.

## User-Agent
- String in the HTTP header typically used to identify the browser that made the request.

## DNS Resolve Order
- **Resolve Order**: DNS > LLMNR > NBT-NS.

## Malware Communication
- Seen in full packet capture during communication with a C2 server.

#gcih

[Sans SEC504]