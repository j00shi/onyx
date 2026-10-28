@echo off
robocopy Z:\ %USERPROFILE%\Code\onyx /E /PURGE /XD target node_modules /MON:1 /R:3 /W:1
