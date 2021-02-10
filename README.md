# MagicUtil

A set of utilities useful on a MagicINFO server compiled to a single binary. Most of the commands have optional JSON output which can be used in other programs.

```bash
MagicINFO Util 1.0
Useful utilities on a MagicINFO server

USAGE:
    magicutil [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    config     Returns one or more configuration properties
    help       Prints this message or the help of the given subcommand(s)
    info       Utilities based on retrieving information from the system
    open       Tries to open the given file
    service    Utilities based on the MagicINFO Windows service
    system     Utilities based on the system itself
    tail       Tries to tail and follow the given file

```
## Why does it exist?
Being a mostly Linux person, clicking through the interface in Windows to search certain configuration files or to tail and follow logs feels less efficient than it does on a Linux machine. Doing all this with a few simple commands in CMD or Powershell helps a lot when doing maintanance on a MagicINFO server.

## What can it do?
The program can do more than the things listed below. This is just an example of what it does.

### Get server information
Getting an overview of the server you're currently working on:
```bash
> magicutil.exe info all --json | jq
{
  "magicinfo": {
    "CONTENTS_HOME": "C:/MagicInfo Premium/runtime/upload",
    "web_url": "https://a.fictional.webserver.com:7002/MagicInfo",
    "wsrm.premiumVersion": "MagicINFO V8 build NA-MICS-20.1010.7"
  },
  "database": {
    "wsrm.dbVendor": "PostgreSQL",
    "wsrm.url": "jdbc:postgresql://localhost:5432/magicinfo",
    "wsrm.username": "admin"
    "wsrm.password": "admin",
  },
  "system": {
    "boardid": "MIXXXXXXXXXX",
    "hwunique": "XXXXXXXXXXXXXXXX",
    "macaddress": "XX:XX:XX:XX:XX:XX"
  },
  "service": {
    "state": "Running",
    "serviceUser": "LocalSystem",
    "startMode": "Auto"
  }
}
```

### Query certain configuration values
```bash
> magicutil.exe config device.log4j.on device.log4j.level --json | jq
{
  "device.log4j.on": "false",
  "device.log4j.level": "INFO"
}
```

### Opening files
It checks if you have Notepad++ installed. If not, it will fall back to the Windows built-in notepad.exe. Note that you don't have to be in the same folder as the file you're requesting. The program knows the location of these files and opens them:
```bash
> magicutil.exe open config.properties
Opened file with Notepad++...
```

### Tail files
All new output from that file will be printed to the terminal:
```bash
> magicutil.exe tail wsrm.log
```

### Manage the Windows service
Doesn't only test if the service is running, but also if it is available and loaded. This will check if the (fairly long) startup procedure has finished and if the interface is available:
```bash
> magicutil.exe service available --json | jq
{
  "available": true
}
```

Restarts the service and additionally wait until the HTTP service is back online:
```bash
> magicutil.exe service restart --available --silent
```

## Install this program

To compile this program yourself execute:
```bash
make windows
```

Or download a version from the [releases page](https://github.com/marknijboer/MagicUtil/releases).