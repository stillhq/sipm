- Functions
- Source Refresh
- Install
  - Dedicated install-fedora
  - Dedicated install-ubuntu
  - Dedicated install-el
  - Dedicated install-debian
  - Dedicated install-arch
  - Dedicated install-aur

- Upgrade individual
- Upgrade system
- Upgrade all
- Remove
- Repositories
    - Flatpak repository
    - Snap repository
    - MultiPM repository
    - add and remove repository
- Search
- Config file / order
- Create Packages

- Local flag
- Daemon to install packages that don't exist

- Flatpak support (flatpak)
  - Implementation difficult: Medium
  - Source generation from flatpak
    - I want a feature to generate proper names like "firefox" from org.mozilla.Firefox
    - Otherwise just make every flatpak generate a package file that it will store locally
- Snap support (snap)
  - Implementation difficult: Medium
  - Source generation from snap
    - just use standard snap names
- DistroBox Containers (distrobox)
  - Implementation difficult: Hard
- Wine Containers (unknown implementation)
  - Implementation difficult: Hard
- Web apps (nativeifer)
  - Implementation difficult: Medium
- AppImage (AppImageUpdater)
  - Implementation difficult: Medium
- OSTree Update
  - Implementation difficult: Easy
  - Likely the easiest to implement, just make it run ostree commands


Conf options:
enabled_sources: Vec<String>,
repositories: Vec<Repo>,
local_install: bool,
ignore_gpg: bool,
ignore_mirrors: bool,
default_local_install: bool,
container_directory: String,
cache_directory: String

Repo Options:
url: String,
mirrorlist: String,
gpg: String,
gpg_check: bool,
enabled: bool