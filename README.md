# HAROLD_RS
Harold but with multithreading and asyc!

HAROLD or Heralding Arrival by a Really Obnoxiously Loud Device is a device that plays snippets of audio files available on audiophiler.csh.rit.edu. Audiophiler will return a random music file out of the ones that you have selected.

Upon scanning your iButton, HAROLD will search ldap for your UID, make an http request to audiophiler to attain the link to the s3 bucket and then download the audio file from the s3 bucket. Once the file is on the Raspberry PI, it is played using VLC media player. While the music is being played, lights will flash to make your entrance that much more special!

This iteration of HAROLD uses the async/await features of Rust, and threading, to try and minimize the silence that occurs while you wait for your audio file to download. This is done by starting the download while a quick message plays notifying you of a successful scan. 
