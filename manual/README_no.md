# WSL Dashboard

<p align="center">
  <img src="../assets/logo/logo.png" width="128" height="128" alt="WSL Dashboard Logo" />
</p>

Et moderne, høyytelses og lettvektig kontrollpanel for administrasjon av WSL-instanser (Windows Subsystem for Linux). Bygget med Rust og Slint for en førsteklasses nativ opplevelse.

---

<p align="left">
  <a href="https://www.rust-lang.org"><img src="https://img.shields.io/badge/rust-v1.92+-orange.svg" alt="Rust" /></a>
  <a href="https://slint.dev"><img src="https://img.shields.io/badge/UI-Slint-blue.svg" alt="Slint" /></a>
  <a href="https://tokio.rs"><img src="https://img.shields.io/badge/Async-Tokio-000000.svg" alt="Tokio" /></a>
  <a href="https://github.com/microsoft/windows-rs"><img src="https://img.shields.io/badge/OS-Windows-0078D6.svg" alt="Windows" /></a>
  <a href="../LICENSE"><img src="https://img.shields.io/badge/license-GPL--3.0-blue.svg" alt="Lisens" /></a>
</p>

I18N :  [English](../README.md) | [简体中文](./README_zh_CN.md) | [繁體中文](./README_zh_TW.md) | [हिन्दी](./README_hi.md) | [Español](./README_es.md) | [Français](./README_fr.md) | [العربية](./README_ar.md) | [বাংলা](./README_bn.md) | [Português](./README_pt.md) | [Русский](./README_ru.md) | [اردو](./README_ur.md) | [Bahasa Indonesia](./README_id.md) | [Deutsch](./README_de.md) | [日本語](./README_ja.md) | [Türkçe](./README_tr.md) | [한국어](./README_ko.md) | [Italiano](./README_it.md) | [Nederlands](./README_nl.md) | [Svenska](./README_sv.md) | [Čeština](./README_cs.md) | [Ελληνικά](./README_el.md) | [Magyar](./README_hu.md) | [עברית](./README_he.md) | Norsk | [Dansk](./README_da.md) | [Suomi](./README_fi.md) | [Slovenčina](./README_sk.md) | [Slovenščina](./README_sl.md) | [Íslenska](./README_is.md)

---

## 📑 Innholdsfortegnelse
- [🌍 Språkstøtte](#-språkstøtte)
- [🚀 Hovedfunksjoner og bruk](#-hovedfunksjoner-og-bruk)
- [⚙️ Konfigurasjon og logger](#️-konfigurasjon-og-logger)
- [🖼️ Skjermbilder](#️-skjermbilder)
- [🎬 Demonstasjon](#-demonstasjon)
- [💻 Systemkrav](#-systemkrav)
- [📦 Installasjonsguide](#-installasjonsguide)
- [🛠️ Teknologistakk og ytelse](#️-teknologistakk-og-ytelse)
- [📄 Lisens](#-lisens)

---

## 🌍 Språkstøtte

Engelsk, Kinesisk (Forenklet), Kinesisk (Tradisjonell), Hindi, Spansk, Fransk, Arabisk, Bengalsk, Portugisisk, Russisk, Urdu, Indonesisk, Tysk, Japansk, Tyrkisk, Koreansk, Italiensk, Nederlandsk, Svensk, Tsjekkisk, Gresk, Ungarsk, Hebraisk, Norsk, Dansk, Finsk, Slovakisk, Slovensk, Islandsk

<p align="left">
  <img src="../assets/flags/us.svg" width="32" title="Engelsk" alt="Engelsk" />
  <img src="../assets/flags/cn.svg" width="32" title="Kinesisk (Forenklet)" alt="Kinesisk (Forenklet)" />
  <img src="../assets/flags/tw.svg" width="32" title="Kinesisk (Tradisjonell)" alt="Kinesisk (Tradisjonell)" />
  <img src="../assets/flags/in.svg" width="32" title="Hindi" alt="Hindi" />
  <img src="../assets/flags/es.svg" width="32" title="Spansk" alt="Spansk" />
  <img src="../assets/flags/fr.svg" width="32" title="Fransk" alt="Fransk" />
  <img src="../assets/flags/sa.svg" width="32" title="Arabisk" alt="Arabisk" />
  <img src="../assets/flags/bd.svg" width="32" title="Bengalsk" alt="Bengalsk" />
  <img src="../assets/flags/pt.svg" width="32" title="Portugisisk" alt="Portugisisk" />
  <img src="../assets/flags/ru.svg" width="32" title="Russisk" alt="Russisk" />
  <img src="../assets/flags/pk.svg" width="32" title="Urdu" alt="Urdu" />
  <img src="../assets/flags/id.svg" width="32" title="Indonesisk" alt="Indonesisk" />
  <img src="../assets/flags/de.svg" width="32" title="Tysk" alt="Tysk" />
  <img src="../assets/flags/jp.svg" width="32" title="Japansk" alt="Japansk" />
  <img src="../assets/flags/tr.svg" width="32" title="Tyrkisk" alt="Tyrkisk" />
  <img src="../assets/flags/kr.svg" width="32" title="Koreansk" alt="Koreansk" />
  <img src="../assets/flags/it.svg" width="32" title="Italiensk" alt="Italiensk" />
  <img src="../assets/flags/nl.svg" width="32" title="Nederlandsk" alt="Nederlandsk" />
  <img src="../assets/flags/se.svg" width="32" title="Svensk" alt="Svensk" />
  <img src="../assets/flags/cz.svg" width="32" title="Tsjekkisk" alt="Tsjekkisk" />
  <img src="../assets/flags/gr.svg" width="32" title="Gresk" alt="Gresk" />
  <img src="../assets/flags/hu.svg" width="32" title="Ungarsk" alt="Ungarsk" />
  <img src="../assets/flags/il.svg" width="32" title="Hebraisk" alt="Hebraisk" />
  <img src="../assets/flags/no.svg" width="32" title="Norsk" alt="Norsk" />
  <img src="../assets/flags/dk.svg" width="32" title="Dansk" alt="Dansk" />
  <img src="../assets/flags/fi.svg" width="32" title="Finsk" alt="Finsk" />
  <img src="../assets/flags/sk.svg" width="32" title="Slovakisk" alt="Slovakisk" />
  <img src="../assets/flags/si.svg" width="32" title="Slovensk" alt="Slovensk" />
  <img src="../assets/flags/is.svg" width="32" title="Islandsk" alt="Islandsk" />
</p>


## 🚀 Hovedfunksjoner og bruk

- **Moderne nativt brukergrensesnitt**: Intuitivt GUI med støtte for mørk/lys modus, glatte animasjoner og høy ytelse drevet av **Skia**.
- **Systemstatusfelt-integrasjon**: Full støtte for minimering til systemfeltet (~10 MB RAM-bruk), dobbeltklikk for å åpne vinduet, og en funksjonell høyreklikksmeny.
- **Intelligent oppstart**: Konfigurer kontrollpanelet til å starte med Windows, minimere til systemfeltet (stille modus med `/silent`), og automatisk avslutning av distribusjoner ved utgang.
- **Omfattende instanskontroll**: Start, stopp, terminer og avregistrer med ett klikk. Statusovervåking i sanntid og detaljert innsikt i diskbruk og filplasseringer.
- **Distribusjonsadministrasjon**: Sett som standard, migrering (flytt VHDX til andre stasjoner), og eksport/kloning til `.tar` eller `.tar.gz`-arkiver.
- **Hurtigintegrasjon**: Umiddelbar start i Terminal, VS Code eller Filutforsker med tilpassbare arbeidsmapper og oppstartsskript-huker.
- **Smart installasjon**: Installer fra Microsoft Store, GitHub eller lokale filer (RootFS/VHDX). Inkluderer en innebygd RootFS-nedlastingshjelper.
- **Global sikkerhet**: Mutex-låser for sikre samtidige migrerings-/sikkerhetskopieringsoperasjoner og automatisk Appx-opprydding ved fjerning.
- **Svært lavt minnebruk**: Høyt optimalisert for effektivitet. Stille oppstart (systemstatusfelt) bruker bare **~10 MB** RAM. Bruk i vindusmodus varierer etter fontkompleksitet: **~18 MB** for standardspråk og **~38 MB** for språk med store tegnsett (kinesisk, japansk, koreansk).


## ⚙️ Konfigurasjon og logger

All konfigurasjon administreres gjennom Innstillinger-visningen:

- Velg standard installasjonsmappe for nye WSL-instanser.
- Konfigurer loggmappe og loggnivå (Error / Warn / Info / Debug / Trace).
- Velg brukergrensesnittspråk eller la det følge systemspråket.
- Bytt mellom lys og mørk modus, og velg om appen skal avslutte WSL automatisk etter operasjoner.
- Konfigurer hvor ofte appen skal se etter oppdateringer (daglig, ukentlig, annenhver uke, månedlig).
- Aktiver automatisk start ved systemoppstart (med automatisk reparasjon av filbane).
- Sett appen til å minimere til systemstatusfeltet ved oppstart.
- Konfigurer lukkeknappen til å minimere til systemstatusfeltet i stedet for å avslutte.

Loggfiler skrives til den konfigurerte loggmappen og kan legges ved når du rapporterer problemer.


## 🖼️ Skjermbilder

### Hjem (Lys og mørk modus)
<p align="center">
  <img src="../assets/screenshot/home.png" width="48%" />
  <img src="../assets/screenshot/home-dark.png" width="48%" />
</p>

### Legg til instans og innstillinger
<p align="center">
  <img src="../assets/screenshot/add.png" width="48%" />
  <img src="../assets/screenshot/settings.png" width="48%" />
</p>

### Om og minimert meny
<p align="center">
  <img src="../assets/screenshot/about.png" width="48%" />
  <img src="../assets/screenshot/collapsed.png" width="48%" />
</p>

## 🎬 Demonstasjon

Nedenfor er en demonstrasjon av WSL Dashboard i aksjon:

![WSL Dashboard Demo](../assets/screenshot/demo.gif)



## 💻 Systemkrav

- Windows 10 eller Windows 11 med WSL aktivert (WSL 2 anbefales).
- Minst én WSL-distribusjon installert, eller tillatelse til å installere nye.
- 64-bit CPU; 4 GB RAM eller mer anbefales for smidig bruk av flere distribusjoner.

## 📦 Installasjonsguide

### Alternativ 1: Last ned ferdigbygget binærfil

Den enkleste måten å komme i gang på er å bruke den ferdigkompilerte versjonen:

1. Gå til siden for [GitHub Releases](https://github.com/owu/wsl-dashboard/releases).
2. Last ned den nyeste `wsldashboard` kjørbare filen for Windows.
3. Pakk ut (hvis den er pakket) og kjør `wsldashboard.exe`.

Ingen installatør er nødvendig; appen er en enkelt bærbar binærfil.

### Alternativ 2: Bygg fra kildekode

Sørg for at du har Rust-verktøykjeden (Rust 1.92+ eller nyere) installert.

1. Klon kodelageret:

   ```powershell
   git clone https://github.com/owu/wsl-dashboard.git
   cd wsl-dashboard
   ```

2. Bygg og kjør:

   - For utvikling:

     ```powershell
     cargo run
     ```
   - Optimalisert produksjonsbygg ved bruk av byggskriptet:

     > Byggskriptet krever verktøykjeden `x86_64-pc-windows-msvc`.

     ```powershell
     .\build\scripts\build.ps1
     ```


## 🛠️ Teknologistakk og ytelse

- **Kjerne**: Implementert i Rust for minnesikkerhet og null-kostnads abstraksjoner.
- **UI-rammeverk**: Slint med høy ytelse **Skia** renderingsmotor.
- **Asynkron kjøretid**: Tokio for ikke-blokkerende systemkommandoer og I/O.
- **Ytelseshøydepunkter**:
  - **Responstid**: Nesten umiddelbar oppstart og statusovervåking av WSL i sanntid.
  - **Effektivitet**: Svært lavt ressursbruk (se [Hovedfunksjoner](#-hovedfunksjoner-og-bruk) for detaljer).
  - **Portabilitet**: Optimalisert produksjonsbygg produserer en enkelt kompakt kjørbar fil.



## 📄 Lisens

Dette prosjektet er lisensiert under GPL-3.0 – se [LICENSE](../LICENSE)-filen for detaljer.

---

Bygget med ❤️ for WSL-fellesskapet.
