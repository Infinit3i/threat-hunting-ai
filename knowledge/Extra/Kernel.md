# Kernel Mode

- #kernel #mode runs 30% faster
- [meltdown & spectre](https://www.cloudflare.com/learning/security/threats/meltdown-spectre/) attacked the kernel
- [crowdstrike](https://medium.com/@jnebos/lessons-to-learn-from-the-crowdstrike-fail-7604ccd55d71) failure was becuase of the kernel
- kernel mode interacts with the hardware
- kernel mode affects the performance
- both windows and linux use monolithic kernels all
    - drivers and services can be loaded without restarting
    - windows has more proprietary drivers in kernel mode which creates instability
    - hardware drivers are meticiously tested
        - drivers will take down your whole system if not ran correctly
- blue screen of death is kernel level problem that cannot be resolved 
    
# User mode
- user mode is in "a sandbox"; the kernel will tell it what to do
- unsafe applications are in user mode for that reason
        - user mode is safe but limited
- user mode will request to elevate something like saving a file and will verify if the request if authentic





[Kernel Mode vs User Mode: Why it Matters, What You Need to Know - Dave's Garage](https://youtu.be/GB7JTXeGcs0?si=DgZjQRdhmaPyVxb0)