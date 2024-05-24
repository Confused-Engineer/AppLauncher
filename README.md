# AppLauncher
Usage:
 - Allow user to have batches of applications in a config file, that can be launched in-app, via API, or via shortcut 
 - Examples:
    - Running 'curl deviceIP:port/api/v1/launch/default' on a seperate device to launch the default batch of apps
    - Running 'AppLauncher.exe games' in a command line to launch the default batch of apps on the same device

Use-Case:
 - For video games you prefer to have Discord, Playnite, and a resource monitor open. You put the path to these files in a section named [Games] in the config file. You can make an app shortcut 'C:\Path\to\AppLauncher.exe games' and next time you want to open all three apps you can select the single shortcut.
 - You have batch scripts that you like to run occasionally, like "winget upgrade --all". You can make a batch script, or a series of batch scripts and launch them all at once with a single button.