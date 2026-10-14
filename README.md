# HyprCenter

# 簡介

一個為 Hyprland 設計的系統設定應用程式。

* 適用 OS：Arch Linux 以及其衍生版本
* 開發
  * 開發語言：Rust
  * GUI框架：Slint

## 想法

本專案將提供一個以 [Hyprland](https://hypr.land/) 作為 Wayland 合成器時使用的系統設定應用程式。

以 [KDE Plasma](https://kde.org/zh-tw/plasma-desktop/) 與 Windows 為例，系統通常提供一個集中式的設定應用程式，可以管理藍牙、Wi-Fi、使用者、顯示器、電源、掛載裝置等系統設定。

|KDE Plasma|Windows|
|-|-|
|![plasma](https://userbase.kde.org/images.userbase/0/02/Systemsettings-plasma5_4.png)|![windows](https://www.windowslatest.com/wp-content/uploads/2021/06/Windows-11-Settings-app.jpg)|

而Hyprland 是一個基於 Wayland 的平鋪式合成器，同時負責視窗管理，但目前並沒有一個整合度較高的圖形化系統設定應用程式，使用者通常需要直接修改設定檔，或透過不同的工具分別進行設定，並且安裝[插件](https://hypr.land/plugins/)，才能將Hyprland設定成方便使用的狀態。

網路上已有 [HyprMod](https://github.com/BlueManCZ/HyprMod) 這類專案，專門提供 Hyprland 的圖形化設定介面，可以設定快捷鍵、動畫、顯示器解析度與更新率、自動啟用服務、環境變數等功能。

|HyprMod||
|-|-|
|![curves](https://raw.githubusercontent.com/BlueManCZ/Hyprmod/refs/heads/main/data/screenshots/curves.png)|![monitors](https://raw.githubusercontent.com/BlueManCZ/Hyprmod/refs/heads/main/data/screenshots/monitors.png)|

不過這類工具主要集中在 Hyprland 本身的設定，並沒有進一步整合其他 Linux 系統設定。因此，本專案希望以類似 KDE Plasma「系統設定」的方式，將 Hyprland 設定與部分常用的 Linux 系統設定整合在同一個應用程式中，降低使用者直接修改設定檔的需求。

因此我們想製做這個應用程式，並為Linux、hyprland、以及hyprland的相關插件提供視覺化的設定支援。

---

另外，Hyprland 社群中也存在許多由使用者製作的 dotfiles，例如 [ML4W](https://www.ml4w.com/os) 的設計，以及 Hyprland 官網中， [Hall of Fame](https://hypr.land/hall_of_fame/) 頁面裡列出的其他 dotfiles。

|ML4W|Hyprland Winter|
|-|-|
|![ML4W OS](https://www.ml4w.com/os/screen-216.jpg)|![Hyprland Winter](https://raw.githubusercontent.com/AmadeusWM/hyprland-winter/refs/heads/master/assets/image_4.png)|

雖然使用現成的 dotfiles 可以快速建立完整的 Hyprland 桌面環境，但部分 dotfiles 會自動安裝大量套件與相關工具，可能增加硬碟空間的使用量。當使用者想移除不需要的套件時，也需要自行確認哪些套件與設定或桌面元件存在相依關係，增加管理上的困難。

因此，我們希望提供一個較為模組化的設定方式，讓使用者可以透過圖形化介面選擇需要的功能，而不需要直接套用完整的 dotfiles。

## 功能

* 初始化引導安裝
  * 引導使用者安裝插件、設定快捷鍵、Hyprland使用引導教學

* Hyprland
  * 視窗與工作區設定
  * 快捷鍵設定
  * 動畫設定
  * 顯示器設定
  * 外觀設定
  * 環境變數
  * 啟動服務
  * 插件瀏覽、安裝、移除
  * 針對特定插件的設定GUI化 (ex. hyprpaper、hyprlauncher、waybar...)

* Linux 系統設定

  * Wi-Fi
  * 藍牙
  * 使用者
  * 電源
  * 顯示器
  * 掛載裝置
  * 硬碟資訊
  * 其他常用系統設定

* 軟體與套件

  * pacman、aur套件瀏覽以及下載、移除功能
  * 已安裝的套件資訊
  * 系統更新

## 參考

- [Hyprland設定教學，漂亮又不佔資源的Linux Wayland合成器 - Ivon的部落格](https://ivonblog.com/posts/hyprland-setup/)
- [Hyprland Wiki](https://wiki.hypr.land/)
- [Hyprland](https://hypr.land)
- [Hyprland - ArchWiki](https://wiki.archlinux.org/title/Hyprland)
- [在 VMware 上跑 Arch Linux + Hyprland：安裝與除錯記錄 - yuuzi](https://blog.yuuzi.cc/posts/notes/vmware-arch-hyprland-troubleshooting/)
- [ML4W](https://ml4w.com/)
- [HyprMod](https://github.com/BlueManCZ/HyprMod)
- [Reddit - r/hyprland](https://www.reddit.com/r/hyprland/)
- [KDE Plasma](https://kde.org/zh-tw/plasma-desktop/)
- [Hyprland 安装与配置超详细教程 (Arch Linux)：含 Waybar 及一键脚本](https://forum.archlinuxcn.org/t/topic/14852)

# 編譯與執行
## 檢查
```rs
cargo check
```

## 執行
```rs
cargo run
```

## 編譯
```rs
cargo build
```