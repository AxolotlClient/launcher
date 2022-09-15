# launcher
AxolotlClient Launcher (v2)

## Setup

### Dependencies
Ubuntu: 
```
$ sudo apt update
$ curl -sL https://deb.nodesource.com/setup_16.x | sudo bash -
$ sudo apt install gcc g++ make nodejs file cargo npm libdbus-1-dev pkg-config libgtk-4-dev libgtk-3-dev libsoup2.4-dev libjavascriptcoregtk-4.0-dev libwebkit2gtk-4.0-dev librsvg2-dev
```

Arch: 
```
$ sudo pacman -S gcc make npm cargo file libsoup pkgconf gtk4 gtk3 webkit2gtk-4.1 librsvg	
```

Fedora: 
```
$ sudo dnf install cargo npm dbus-devel pkgconf-pkg-config openssl-devel pango-devel cairo-gobject-devel libsoup-devel javascriptcoregtk4.0-devel gdk-pixbuf2-devel gtk4-devel gtk3-devel webkit2gtk4.0-devel librsvg2-devel file
```

```
$ git clone https://github.com/AxolotlClient/launcher
& cd launcher
$ npm install @tauri-apps/api
$ cargo install tauri-cli
$ cargo tauri dev
```
