# instalar_office

Small rust project to deploy Office by Volume. It allows you to select the channel, download the Office Deployment Tool executable and generate the configuration xml file and then proceed to run the installer with that configuration.

If you select a channel other than "O365ProPlusRetail" and in the folder where the program runs it finds the "clave.txt" file, enter that key in the xml file, if not, ask for the product key via console.
