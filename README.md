# CAB - Camera Auto Brightness
Cab is a program that runs in the background and sets the brightness of your computer screen using the data from a connected camera.
This is useful for computers that do have a built-in autobrightness feature from the operating system but it's not working because there is no brightness sensor on the computer.
Note that some computers reduce the max brightness level when on power saving mode.

## Task priorities
1. Get environment brightness level from camera (maybe use this, https://crates.io/crates/nokhwa) ✅
2. Set computer screen brightness level (maybe use this, https://crates.io/crates/brightness) ✅
3. Add Taskbar (Tray icon, maybe use this https://crates.io/crates/tray-item) for closing the program and (later for) settings. A subtask here is to make an icon/logo
4. Make program run in the background
5. Add power efficient way for CAB to regularly check the brightness.
6. Add setting adjustability/power use where user can set how long apart (in minutes?) CAB checks the environment brightness level (and adjusts the screen brightness accordingly)
7. Add setting to choose between cameras (if the computer has more than 1)
8. Add logging capabilities (Maybe use this: https://crates.io/crates/log)

## Current usage
Currently to use this program you need to download the cab.exe file, save it to your computer and add a scheduler through the task scheduler (Here is a link that explains how to do that, you are smart and can figure out which checkboxes you need and such: https://www.ibm.com/docs/en/datacap/9.1.9?topic=applications-scheduling-maintenance-application-run-automatically). I configured mine to run every 5 minutes (since it's the highest frequency available to me on the task scheduler) and so far the problem has been that the task scheduler has not been running the cab properly, on time but at least it runs sometimes and takes very little space and time when running ^_^

## Contributing to the project
Feel free to contribute to the project and make it better, add issues and such, this project is hosted here on github and anyone can contribute. Huldar is a busy person though so he might not respond right away or do the things you ask but you can always do them yourself and submit a pull request and if you really want, contact me about becoming a collaborator.

### Releases
Releases are set up according to the way Kahan data studios explain in this video: https://www.youtube.com/watch?v=Ob9llA_QhQY

## Note about rlight
Rlight
https://github.com/theunixer/rlight
Is a program that uses the camera as a brightness sensor to automatically set the computer screen brightness just like CAB. However rlight only works on linux and since Huldar has a windows OS he decided to make another program after searching online for something already made.

## Created
2024-02-13 by Huldar