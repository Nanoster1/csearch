# csearch
> **CLI Utility for the opening your browser from the terminal**

[![Crates.io](https://img.shields.io/crates/v/csearch?style=flat-square)](https://crates.io/crates/csearch)

## Examples
```bash
csearch "Some query here" -b Default -s Google
```
![alt](https://raw.githubusercontent.com/Nanoster1/resources/main/.github/images/Example.png)
## Choose Browser
It works on [webbrowser library](https://docs.rs/webbrowser/latest/webbrowser/)\
On **Linux**, you can set the environment variable `BROWSER`\
\
For example in *Bash*:
```bash 
export BROWSER="/usr/bin/links"
``` 
Or in *PowerShell*
```powershell
$env:BROWSER = "/usr/bin/links"
```
It's also working with browsers like [*Links*](http://links.twibright.com/user_en.html)
## Completions
You can generate common completions for your shell\
\
For example in *PowerShell*
```powershell
New-Item -Path ~/.cargo/scripts/ -Type Directory
csearch --completions Bash >> ~/.cargo/scripts/csearch.ps1
Add-Content -Path $PROFILE -Value "Invoke-Expression -Command ~/.cargo/scripts/*.ps1"
```