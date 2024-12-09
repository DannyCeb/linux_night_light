# Linux Night Light

This program adjusts the screen gamma values based on the time of day. It switches to night light mode during specified nighttime hours and to normal light mode during the day.

## Requirements

- `xrandr` must be installed on your system.
- A compatible Linux window manager (e.g., i3, GNOME, KDE).

## Installation

1. Ensure `xrandr` is installed:
    ```sh
    sudo apt-get install x11-xserver-utils
    ```

2. Clone the repository and build the project:
    ```sh
    git clone https://github.com/yourusername/linux_night_light.git
    cd linux_night_light
    cargo build --release
    ```

3. Move the binary to your preferred location, e.g., `/home/yourusername/bin`:
    ```sh
    mv target/release/linux_night_light /home/yourusername/bin/
    ```

## Usage

To use the program, you need to provide the gamma values for day and night, the hours considered nighttime, and the name of your monitor. You can find the name of your monitor by running `xrandr` in your terminal.

Example command:
```sh
/home/yourusername/bin/linux_night_light 1.0:1.0:1.0 1.0:0.8:0.7 19 7 eDP
```

-1.0:1.0:1.0 is the gamma value for the day.

-1.0:0.8:0.7 is the gamma value for the night.

-19 is the hour when nighttime starts (7 PM).

-7 is the hour when nighttime ends (7 AM).

-eDP is the name of your monitor (use the output of xrandr to find your monitor's name).

Configuration for i3 Window Manager
To ensure the program runs each time you log in to i3, add the following line to your i3 configuration file (~/.config/i3/config):

```
exec --no-startup-id /home/yourusername/bin/linux_night_light 1.0:1.0:1.0 1.0:0.8:0.7 19 7 eDP
```
After saving the file, restart i3:
```sh
i3-msg restart
````


This `README.md` file provides an overview of the project, installation instructions, usage examples, and configuration steps for different Linux window managers. Feel free to customize it further to suit your repository's needs. If you need more help, let me know!
