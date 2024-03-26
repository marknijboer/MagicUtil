# MagicUtil

A set of utilities useful on a MagicINFO server compiled to a single binary.
Most of the commands have optional JSON output which can be used in other
programs.

```
MagicUtil 0.1.12
Released under the MIT license.

Useful utilities on a Samsung MagicINFO server for sysadmin tasks.

USAGE:
    magicutil <SUBCOMMAND>

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information

SUBCOMMANDS:
    bcrypt     Utilities based on MagicINFO's bcrypt hashing algorithm used to store password
    config     Get, set, replace or remove properties from MagicINFO's main config.properties
               file
    help       Print this message or the help of the given subcommand(s)
    info       Utilities based on retrieving information from the system
    open       Tries to open the given file
    service    Utilities based on the MagicINFO Windows service
    system     Query system properties used in MagicINFO
    tail       Tries to tail and follow the given file
```
## Why does it exist?
Being a mostly Linux person, clicking through the interface in Windows to search
certain configuration files or to tail and follow logs feels less efficient
than it does on a Linux machine. Doing all this with a few simple commands in
CMD or Powershell helps a lot when doing maintenance on a MagicINFO server.

## What can it do?
The program can do more than the things listed below. This is just an example of
what it does.

### Get server information
Getting an overview of the server you're currently working on:
```powershell
PS C:\> magicutil info all --json | jq
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
```powershell
PS C:\> magicutil config get device.log4j.on device.log4j.level --json | jq
{
  "device.log4j.on": "false",
  "device.log4j.level": "INFO"
}
```

### Query encrypted configuration values
```powershell
PS C:\> magicutil config get wsrm.username wsrm.password --decrypt --json | jq
{
  "wsrm.username": "postgres",
  "wsrm.password": "password"
}
```

### Overlay two configuration files to generate a new configuration file
```powershell
PS C:\> magicutil config overlay ./template.properties ./overlay.properties | Out-File -FilePath ./config.properties
```

### Setting certain configuration values
```powershell
PS C:\> magicutil config set listen.port 7001
PS C:\> magicutil config set wsrm.username postgres --encrypt
```

### Opening files
It checks if you have Notepad++ installed. If not, it will fall back to the
Windows built-in notepad.exe. Note that you don't have to be in the same folder
as the file you're requesting. The program knows the location of these files and
opens them:
```powershell
PS C:\> magicutil open config.properties
Opened file with Notepad++...
```

### Tail files
All new output from that file will be printed to the terminal:
```powershell
PS C:\> magicutil tail wsrm.log
```

### Hash passwords
A string of text can be converted to a password hash, as used in MagicINFO's database to authenticate a user.
```powershell
PS C:\> magicutil bcrypt hash testpassword
$2a$10$91MkpP94Jhd6Uhy2gZlDxOMpvPo04zX5uekxMp78IENSp9pYEJf9e
```

### Manage the Windows service
Doesn't only test if the service is running, but also if it is available and
loaded. This will check if the (fairly long) startup procedure has finished and
if the interface is available:
```powershell
PS C:\> magicutil service available --json | jq
{
  "available": true
}
```

Restarts the service and additionally wait until the HTTP service is back online:
```powershell
PS C:\> magicutil service restart --available --silent
```

After some other service (re)started MagicINFO, it can wait until the HTTP
service is back online:
```powershell
PS C:\> magicutil service wait --available
```

## Install this program

### Dependencies
Make sure you have Rust installed, there is a oneliner-installer on their [website](https://www.rust-lang.org/tools/install):
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

To cross-compile to other OS's and architectures we need [cross](https://github.com/cross-rs/cross): 
```bash
cargo install cross
```

Cross depends on Docker to build programs. If you don't have Docker yet, make
sure to install it. The installation instructions for Docker are different for every
OS, so they won't be listed here.

### Compile
To compile this program yourself execute:

```bash
make windows
```

### Download
Or download a version from the [releases page](https://github.com/marknijboer/MagicUtil/releases).