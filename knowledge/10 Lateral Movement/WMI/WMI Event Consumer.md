# WMI Event Consumer Backdoors

#CyberSecurity
#EventMonitoring
#SystemAdministration
#PersistenceTechniques
#PowerShellScripting

- **WMI Event Triggers**
  - Event triggers can be used to execute arbitrary code when specific conditions are met.
  - Commonly used for persistence and lateral movement.

- **WMI Event Filters**
  - Filters define the conditions under which the event triggers.
  - Example: A filter can be set to trigger when a specific process starts.

- **WMI Event Consumers**
  - Consumers define the actions to be taken when the event is triggered.
  - Example: A consumer can execute a script or a command.

- **WMI Event Bindings**
  - Bindings link filters and consumers.
  - Example: A binding can link a filter that detects process start to a consumer that executes a script.

## DIFF

WMI event consumer backdoors are a stealthy method for attackers to maintain persistence on a compromised system. By using WMI event consumers, attackers can execute malicious code in response to specific system events, making detection and removal challenging.

- **Detection**
  - Monitoring WMI activity can help detect suspicious event consumers.
  - Tools like Sysmon can be configured to log WMI activity.

- **Mitigation**
  - Regular audits of WMI filters and consumers can help identify unauthorized entries.
  - Disabling unnecessary WMI components can reduce the attack surface.

Sure! Here's the content from your image in Markdown format:

Chris Glyer gave an excellent presentation describing the (limited) forensic artifacts left behind by WMI Event Consumers. The Sysinternals tool Autoruns and the Kansa PowerShell framework both identify WMI event filters and consumers. The PowerShell cmdlet "Get-WmiObject" can also be used as a native means to identify and help remove suspicious entries.

The contents of a sample malicious MOF file follows:

```
mof
#PRAGMA AUTORECOVER
#PRAGMA NAMESPACE("\\\\.\\root\\subscription")

instance of __EventFilter as $Filter
{
    Name = "SCM Event Filter";
    QueryLanguage = "WQL";
    Query = "SELECT * FROM __InstanceModificationEvent WITHIN 60 WHERE TargetInstance ISA 'Win32_Service' AND TargetInstance.Name='sens'";
};

instance of CommandLineEventConsumer as $Consumer
{
    Name = "SCM Event Consumer";
    ExecutablePath = "c:\\windows\\system32\\cmd.exe";
    CommandLineTemplate = "net stop sens && net start sens";
};

instance of __FilterToConsumerBinding
{
    Filter = $Filter;
    Consumer = $Consumer;
};
```

References:
1. [Monitoring Events](http://msdn.microsoft.com/en-us/library/aa394531%28v=vs.85%29.aspx)
2. [Stuxnet Under the Microscope (PDF)](http://www.f-secure.com/weblog/archives/Stuxnet_Under_the_Microscope.pdf)
3. [There's Something About WMI](http://blogs.msdn.com/b/cclayton/archive/2011/09/30/there-s-something-about-wmi.aspx)
4. [Kansa and WMI Event Consumers](http://forensicKB.com/2013/06)