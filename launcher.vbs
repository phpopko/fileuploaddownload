Dim sDir, sExe
sDir = CreateObject("Scripting.FileSystemObject").GetParentFolderName(WScript.ScriptFullName)
sExe = sDir & "\target\release\transfer.exe"
CreateObject("WScript.Shell").Run Chr(34) & sExe & Chr(34), 0, False
