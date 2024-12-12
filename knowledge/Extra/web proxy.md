#dwell_time - this is the time between attacker getting in to victim knows about attack
---
- typical dwell time in 2021 is 3 weeks
- 47% of victims get notified by outside source
- close to 10% of victims have a dwell time over a year

#web_proxys - prevent users from accessing malicious sites and keep logs
---
- watches http and https
- can function as reverse proxy
- content transaction logs & cached data itself

types of #web_proxys (572.1 p26)
---
- [squid](https://www.squid-cache.org/) - #open_source, easy to use
- [nginx](https://nginxproxymanager.com/) - #open_source, web server that can also be a web proxy
- [apache traffic server](https://httpd.apache.org/docs/2.4/howto/reverse_proxy.html) - #open_source, web server that can also be a web proxy
- [symantec web filter](https://docs.broadcom.com/doc/webfilter-en) - #commercial, used by corporate enterprise
- [forcepoint](https://www.forcepoint.com/) - #commercial, used in large scale enviroments
- [zscaler](https://www.zscaler.com/) - #commercial, cloud based

#squid_proxy 
3 main forensic relevant elements
- configuration files - /etc/squid/squid.conf
    - default config - /etc/squid/squid.conf
- log files - /varlog/squid/*
    - basically an access roster of who accessed
- cache data - /var/spool/squid

- can incorporate cache policy that will keep .exe files longer than .html files
- uses port - TCP/3128
    - should be changed

- can create access list based on IP, time and day

#gcfa