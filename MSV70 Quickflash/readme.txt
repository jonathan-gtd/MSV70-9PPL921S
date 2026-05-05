MSV70 Quickflash - MSV70 Flasher
https://www.bimmertuningtools.com 

===========================

Requirements:
.NET Framework 4.6 or later
FTDI Driver - https://ftdichip.com/drivers/d2xx-drivers/
FT232RL Cable

Windows 7 users must have the below updates installed: 
https://catalog.update.microsoft.com/search.aspx?q=kb4474419 
https://catalog.update.microsoft.com/search.aspx?q=4490628

If you are experiencing a "path" related error message, copy the FTD2XX_NET.dll to c:\windows\system32 then try running the program with your cable plugged in both with and without the DLL in Quickflash's director location. 

===========================

Using Quickflash:

FIRST AND FOREMOST, you must set your cable's latency to 1ms in device manager. 
Device Manager>Ports(COM&LPT)>Right click your cables device>Properties>Port Settings>Latency Timer (msec): Change 16 to 1. Ok, Ok, close device manager.

Functions will remain disabled until your cable is properly detected and installed. If Quickflash reamins disabled after connecting your cable, there is a problem with detecting the cable, wrong cable, or cable drivers installed imporperly. 

K-line check box can be used for bench flashing via K line. Use K-line for Z4 and bench flashing.

Reading ECU-
Use the read tab. You can read the partial (tune/calibration) or the full ecu.  

Writing to ECU- 
Use the Write tab. Use the Write tab. You can write partial (calibration) section or full (program + calibration) section. In order to flash modified files, you must first unlock the ECU. If you load a stock full BIN, the application will ask you if you want to use the file to unlock or restore the ECU. If you load a modified full BIN, flash full will unlock the ecu. MO3 checksums- This is an extra checksum for the torque ceiling tables. When editing these tables, load the full bin into the application to correct the MO3 checksum within the tune. If not editing these tables, then you only need to load your tune for flashing. Standard checksums are always automatically corrected.   

EWS Patch - To disable EWS, you MUST first do a full flash of your full bin so that RSA Detected is True. Then you can load your full bin and check the patch ews check box and flash full again to disable EWS. 

Diag Tab allows you to clear diagnostic trouble codes(DTCs) and adaptations. It also allows you to read DTCs. 

