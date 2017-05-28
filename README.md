# Notification

A (Windows-only) program I made for a friend that create a pop-up each x seconds to remind her of something (knowing her, it's most likely that she should go to bed!)

Written in Rust, first "project" in it. Had no idea what I was doing

Tray/Application icon made by EpicCoders from www.flaticon.com

## Usage

Simply launch the .exe. A pop-up will appear each x minutes (See below for configuration)

To exit the program, right-click the tray icon and press Exit

## Configuration

On first launch a .ini configuration file will be generated in the same folder the program is in. There's three properties available :

**delay** : In minutes, the time between each pop-up (Default : 30)

**content** : The content of the notification (Default : Hey!)

**title** : The title of the pop-up (Default : Notification)

The configuration is loaded at launch and reloaded when the user press Ok on the pop-up
