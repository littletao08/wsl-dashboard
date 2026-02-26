# WSL Dashboard

<p align="center">
  <img src="../assets/logo/logo.png" width="128" height="128" alt="WSL Dashboard Logó" />
</p>

Egy modern, nagy teljesítményű és könnyű vezérlőpult a WSL (Windows Subsystem for Linux) példányok kezeléséhez. Rust és Slint technológiával készült a prémium natív élmény érdekében.

---

<p align="left">
  <a href="https://www.rust-lang.org"><img src="https://img.shields.io/badge/rust-v1.92+-orange.svg" alt="Rust" /></a>
  <a href="https://slint.dev"><img src="https://img.shields.io/badge/UI-Slint-blue.svg" alt="Slint" /></a>
  <a href="https://tokio.rs"><img src="https://img.shields.io/badge/Async-Tokio-000000.svg" alt="Tokio" /></a>
  <a href="https://github.com/microsoft/windows-rs"><img src="https://img.shields.io/badge/OS-Windows-0078D6.svg" alt="Windows" /></a>
  <a href="../LICENSE"><img src="https://img.shields.io/badge/license-GPL--3.0-blue.svg" alt="Licenc" /></a>
</p>

I18N :  [English](../README.md) | [简体中文](./README_zh_CN.md) | [繁體中文](./README_zh_TW.md) | [हिन्दी](./README_hi.md) | [Español](./README_es.md) | [Français](./README_fr.md) | [العربية](./README_ar.md) | [বাংলা](./README_bn.md) | [Português](./README_pt.md) | [Русский](./README_ru.md) | [اردو](./README_ur.md) | [Bahasa Indonesia](./README_id.md) | [Deutsch](./README_de.md) | [日本語](./README_ja.md) | [Türkçe](./README_tr.md) | [한국어](./README_ko.md) | [Italiano](./README_it.md) | [Nederlands](./README_nl.md) | [Svenska](./README_sv.md) | [Čeština](./README_cs.md) | [Ελληνικά](./README_el.md) | Magyar | [עברית](./README_he.md) | [Norsk](./README_no.md) | [Dansk](./README_da.md) | [Suomi](./README_fi.md) | [Slovenčina](./README_sk.md) | [Slovenščina](./README_sl.md) | [Íslenska](./README_is.md)

---

## 📑 Tartalomjegyzék
- [🌍 Támogatott nyelvek](#-támogatott-nyelvek)
- [🚀 Főbb jellemzők és használat](#-főbb-jellemzők-és-használat)
- [⚙️ Beállítások és naplók](#️-beállítások-és-naplók)
- [🖼️ Képernyőképek](#️-képernyőképek)
- [🎬 Működési bemutató](#-működési-bemutató)
- [💻 Rendszerkövetelmények](#-rendszerkövetelmények)
- [📦 Telepítési útmutató](#-telepítési-útmutató)
- [🛠️ Technológiai háttér és teljesítmény](#️-technológiai-háttér-és-teljesítmény)
- [📄 Licenc](#-licenc)

---

## 🌍 Támogatott nyelvek

Angol, egyszerűsített kínai, hagyományos kínai, hindi, spanyol, francia, arab, bengáli, portugál, orosz, urdu, indonéz, német, japán, török, koreai, olasz, holland, svéd, cseh, görög, magyar, héber, norvég, dán, finn, szlovák, szlovén, izlandi.

<p align="left">
  <img src="../assets/flags/us.svg" width="32" title="Angol" alt="Angol" />
  <img src="../assets/flags/cn.svg" width="32" title="Kínai (Egyszerűsített)" alt="Kínai (Egyszerűsített)" />
  <img src="../assets/flags/tw.svg" width="32" title="Kínai (Hagyományos)" alt="Kínai (Hagyományos)" />
  <img src="../assets/flags/in.svg" width="32" title="Hindi" alt="Hindi" />
  <img src="../assets/flags/es.svg" width="32" title="Spanyol" alt="Spanyol" />
  <img src="../assets/flags/fr.svg" width="32" title="Francia" alt="Francia" />
  <img src="../assets/flags/sa.svg" width="32" title="Arab" alt="Arab" />
  <img src="../assets/flags/bd.svg" width="32" title="Bengáli" alt="Bengáli" />
  <img src="../assets/flags/pt.svg" width="32" title="Portugál" alt="Portugál" />
  <img src="../assets/flags/ru.svg" width="32" title="Orosz" alt="Orosz" />
  <img src="../assets/flags/pk.svg" width="32" title="Urdu" alt="Urdu" />
  <img src="../assets/flags/id.svg" width="32" title="Indonéz" alt="Indonéz" />
  <img src="../assets/flags/de.svg" width="32" title="Német" alt="Német" />
  <img src="../assets/flags/jp.svg" width="32" title="Japán" alt="Japán" />
  <img src="../assets/flags/tr.svg" width="32" title="Török" alt="Török" />
  <img src="../assets/flags/kr.svg" width="32" title="Koreai" alt="Koreai" />
  <img src="../assets/flags/it.svg" width="32" title="Olasz" alt="Olasz" />
  <img src="../assets/flags/nl.svg" width="32" title="Holland" alt="Holland" />
  <img src="../assets/flags/se.svg" width="32" title="Svéd" alt="Svéd" />
  <img src="../assets/flags/cz.svg" width="32" title="Cseh" alt="Cseh" />
  <img src="../assets/flags/gr.svg" width="32" title="Görög" alt="Görög" />
  <img src="../assets/flags/hu.svg" width="32" title="Magyar" alt="Magyar" />
  <img src="../assets/flags/il.svg" width="32" title="Héber" alt="Héber" />
  <img src="../assets/flags/no.svg" width="32" title="Norvég" alt="Norvég" />
  <img src="../assets/flags/dk.svg" width="32" title="Dán" alt="Dán" />
  <img src="../assets/flags/fi.svg" width="32" title="Finn" alt="Finn" />
  <img src="../assets/flags/sk.svg" width="32" title="Szlovák" alt="Szlovák" />
  <img src="../assets/flags/si.svg" width="32" title="Szlovén" alt="Szlovén" />
  <img src="../assets/flags/is.svg" width="32" title="Izlandi" alt="Izlandi" />
</p>


## 🚀 Főbb jellemzők és használat

- **Modern natív felület**: Intuitív grafikus felhasználói felület sötét/világos mód támogatással, folyamatos animációkkal és nagy teljesítményű, **Skia** alapú megjelenítéssel.
- **Rendszertálca integráció**: Teljes körű támogatás a tálcára kicsinyítéshez (~10 MB RAM használat), dupla kattintás az ablak megjelenítéséhez/elrejtéséhez, valamint funkcionális jobb gombos menü.
- **Intelligens indítás**: Konfigurálható a Windows-zal együtt történő indulás, tálcára kicsinyített indítás (csendes mód a `/silent` opcióval), és a disztribúciók automatikus leállítása kilépéskor.
- **Átfogó példánykezelés**: Indítás, leállítás, kényszerített leállítás és regisztráció törlése egyetlen kattintással. Valós idejű állapotfigyelés, részletes adatok a lemezhasználatról és a fájlok helyéről.
- **Disztribúció-kezelés**: Alapértelmezetté tétel, migrálás (VHDX mozgatása más meghajtókra), valamint exportálás/klónozás `.tar` vagy `.tar.gz` archívumokba.
- **Gyors integráció**: Azonnali indítás Terminálba, VS Code-ba vagy Fájlkezelőbe, testreszabható munkakönyvtárakkal és indítási parancsfájl-kezeléssel.
- **Okos telepítés**: Telepítés Microsoft Store-ból, GitHub-ról vagy helyi fájlokból (RootFS/VHDX). Beépített RootFS letöltési segéd.
- **Biztonság**: Mutex zárolás a biztonságos egyidejű migrálási/mentési műveletekhez, és automatikus Appx tisztítás eltávolításkor.
- **Rendkívül alacsony memóriaigény**: Hatékonyságra optimalizálva. A tálcán futó (csendes) mód csupán **~10 MB** RAM-ot használ. Az ablakos mód használata a betűtípusok összetettségétől függően változik: **~18 MB** a standard nyelveknél és **~38 MB** a nagy karakterkészletű nyelveknél (kínai, japán, koreai).


## ⚙️ Beállítások és naplók

Minden konfiguráció a Beállítások nézetben kezelhető:

- Válassza ki az alapértelmezett telepítési könyvtárat az új WSL példányokhoz.
- Konfigurálja a naplózási könyvtárat és a naplózási szintet (Error / Warn / Info / Debug / Trace).
- Válassza ki a felhasználói felület nyelvét, vagy hagyja, hogy kövesse a rendszer nyelvét.
- Váltson sötét módra, és állítsa be, hogy az alkalmazás automatikusan leállítsa-e a WSL-t a műveletek után.
- Állítsa be a frissítések ellenőrzésének gyakoriságát (naponta, hetente, kéthetente, havonta).
- Engedélyezze az automatikus indítást a rendszer indításakor (automatikus elérési út javítással).
- Állítsa be az alkalmazást, hogy indításkor a rendszertálcára kicsinyítsen.
- Konfigurálja a bezárás gombot, hogy a tálcára kicsinyítsen a programból való kilépés helyett.

A naplófájlok a konfigurált könyvtárba íródnak, és csatolhatók hiba jelentésekor.


## 🖼️ Képernyőképek

### Főoldal (Világos és sötét mód)
<p align="center">
  <img src="../assets/screenshot/home.png" width="48%" />
  <img src="../assets/screenshot/home-dark.png" width="48%" />
</p>

### Példány hozzáadása és Beállítások
<p align="center">
  <img src="../assets/screenshot/add.png" width="48%" />
  <img src="../assets/screenshot/settings.png" width="48%" />
</p>

### Névjegy és összecsukott menü
<p align="center">
  <img src="../assets/screenshot/about.png" width="48%" />
  <img src="../assets/screenshot/collapsed.png" width="48%" />
</p>

## 🎬 Működési bemutató

Az alábbiakban megtekintheti a WSL Dashboard működését:

![WSL Dashboard Demo](../assets/screenshot/demo.gif)



## 💻 Rendszerkövetelmények

- Windows 10 vagy Windows 11 engedélyezett WSL-lel (WSL 2 javasolt).
- Legalább egy telepített WSL disztribúció, vagy jogosultság újak telepítéséhez.
- 64 bites CPU; 4 GB RAM vagy több javasolt a több disztribúció zökkenőmentes használatához.

## 📦 Telepítési útmutató

### 1. opció: Előre fordított bináris letöltése

A legegyszerűbb módja a kezdésnek az előre fordított kiadás használata:

1. Látogasson el a [GitHub Releases](https://github.com/owu/wsl-dashboard/releases) oldalra.
2. Töltse le a legújabb `wsldashboard` futtatható fájlt Windowshoz.
3. Csomagolja ki (ha szükséges) és futtassa a `wsldashboard.exe` fájlt.

Nincs szükség telepítőre; az alkalmazás egyetlen hordozható bináris fájl.

### 2. opció: Fordítás forráskódból

Győződjön meg arról, hogy a Rust eszközkészlet (Rust 1.92+ vagy újabb) telepítve van.

1. Klónozza a tárolót:

   ```powershell
   git clone https://github.com/owu/wsl-dashboard.git
   cd wsl-dashboard
   ```

2. Fordítás és futtatás:

   - Fejlesztéshez:

     ```powershell
     cargo run
     ```
   - Optimalizált kiadási verzió készítése a build szkripttel:

     > A build szkripthez az `x86_64-pc-windows-msvc` eszközkészlet szükséges.

     ```powershell
     .\build\scripts\build.ps1
     ```


## 🛠️ Technológiai háttér és teljesítmény

- **Mag**: Rust nyelven valósítva meg a memóriabiztonság és a zéró költségű absztrakció érdekében.
- **UI Framework**: Slint nagy teljesítményű **Skia** megjelenítő motorral.
- **Async Runtime**: Tokio a nem blokkoló rendszerparancsokhoz és I/O műveletekhez.
- **Teljesítmény főbb jellemzői**:
  - **Válaszkészség**: Közel azonnali indítás és valós idejű WSL állapotfigyelés.
  - **Hatékonyság**: Rendkívül alacsony erőforrás-használat (részletekért lásd: [Főbb jellemzők](#-főbb-jellemzők-és-használat)).
  - **Hordozhatóság**: Az optimalizált kiadási verzió egyetlen kompakt futtatható fájlt eredményez.



## 📄 Licenc

Ez a projekt a GPL-3.0 licenc alatt áll – részletekért lásd a [LICENSE](../LICENSE) fájlt.

---

Készült ❤️-vel a WSL közösség számára.
