@echo off
robocopy Z:\ %USERPROFILE%\Code\onyx /MIR /FFT /XD target node_modules /R:1 /W:1 /NFL /NDL /NP
