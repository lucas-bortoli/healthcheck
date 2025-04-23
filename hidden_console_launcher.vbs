' update the directories below and copy this file to shell:startup

Set WshShell = CreateObject("WScript.Shell")
WshShell.CurrentDirectory = "C:\Users\Lucas\Binaries" 
WshShell.Run "C:\Users\Lucas\Binaries\healthcheck.exe", 0, False ' 0 = hidden window
Set WshShell = Nothing