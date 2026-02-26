For the architecture of the application, what we are going for is modular approach with ability to create your own utility and connect it to the base Power-Utility application.

There will be 3 primary components:
1. The Daemon : this is will be an always active process running in the background primarily listening to key strokes to the activate the utilities.
2. Modules : the sub application which will be core to the appplication and called bundled with the application including launcher.
3. Plug-Ins : enables other users to create their custom extensions and plug it to the main process.

```
powertoys-linux/
├── Cargo.toml                  # Workspace root
├── README.md
├── LICENSE
├── .gitignore
│
├── docs/
│   ├── architecture.md
│   ├── plugin-api.md
│   ├── security.md
│   └── user-guide.md
│
├── scripts/
│   ├── install.sh
│   ├── uninstall.sh
│   ├── security-audit.sh
│   └── build-release.sh
│
├── config/
│   └── default.toml
│
├── assets/
│   ├── icons/
│   │   ├── tray-icon.png
│   │   ├── colorpicker.png
│   │   ├── launcher.png
│   │   ├── taskmanager.png
│   │   └── zones.png
│   └── desktop/
│       └── powertoys.desktop
│
└── crates/
    │
    ├── daemon/                     # Background service
    │   ├── Cargo.toml
    │   └── src/
    │       ├── main.rs
    │       ├── hotkeys.rs
    │       ├── tray.rs
    │       ├── plugin_loader.rs
    │       └── config.rs
    │
    ├── core/                       # Shared library
    │   ├── Cargo.toml
    │   └── src/
    │       ├── lib.rs
    │       ├── plugin_api.rs
    │       ├── config.rs
    │       ├── overlay.rs
    │       ├── x11/
    │       │   ├── mod.rs
    │       │   ├── connection.rs
    │       │   ├── screen.rs
    │       │   └── window.rs
    │       └── wayland/
    │           └── mod.rs
    │
    ├── providers/                  # Data source wrappers
    │   ├── proc/
    │   │   ├── Cargo.toml
    │   │   └── src/
    │   │       ├── lib.rs
    │   │       ├── cpu.rs
    │   │       ├── memory.rs
    │   │       ├── processes.rs
    │   │       └── network.rs
    │   │
    │   ├── nvidia/
    │   │   ├── Cargo.toml
    │   │   └── src/
    │   │       └── lib.rs
    │   │
    │   └── sensors/
    │       ├── Cargo.toml
    │       └── src/
    │           └── lib.rs
    │
    └── modules/                    # Feature plugins
        │
        ├── colorpicker/
        │   ├── Cargo.toml
        │   └── src/
        │       └── main.rs
        │
        ├── launcher/
        │   ├── Cargo.toml
        │   └── src/
        │       ├── main.rs
        │       ├── indexer.rs
        │       └── search.rs
        │
        ├── taskmanager/
        │   ├── Cargo.toml
        │   └── src/
        │       ├── main.rs
        │       └── ui.rs
        │
        ├── zones/
        │   ├── Cargo.toml
        │   └── src/
        │       ├── main.rs
        │       ├── layout.rs
        │       ├── editor.rs
        │       └── snapping.rs
        │
        └── alwaysontop/
            ├── Cargo.toml
            └── src/
                └── main.rs
```

## Componenets of the Architecture

### 1. Daemon (crates/daemon)

The Background process which run after the system as been booted.
As of now the only purpose it serves is to listen to keys and activate the corresponding utility.
This includes `main.rs`, `hotkeys.rs`, `tray.rs`, `plugin_loader.rs` and `config.rs`.

### 2. Core Library (crates/power-core)

The core library will consist of the shared code used by all the modules and or plugins.
