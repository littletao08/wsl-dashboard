# WSL Dashboard

<p align="center">
  <img src="../assets/logo/logo.png" width="128" height="128" alt="WSL Dashboard Logotyp" />
</p>

En modern, högpresterande och lättviktig kontrollpanel för hantering av WSL-instanser (Windows Subsystem for Linux). Byggd med Rust och Slint för en förstklassig nativ upplevelse.

---

<p align="left">
  <a href="https://www.rust-lang.org"><img src="https://img.shields.io/badge/rust-v1.92+-orange.svg" alt="Rust" /></a>
  <a href="https://slint.dev"><img src="https://img.shields.io/badge/UI-Slint-blue.svg" alt="Slint" /></a>
  <a href="https://tokio.rs"><img src="https://img.shields.io/badge/Async-Tokio-000000.svg" alt="Tokio" /></a>
  <a href="https://github.com/microsoft/windows-rs"><img src="https://img.shields.io/badge/OS-Windows-0078D6.svg" alt="Windows" /></a>
  <a href="../LICENSE"><img src="https://img.shields.io/badge/license-GPL--3.0-blue.svg" alt="Licens" /></a>
</p>

I18N :  [English](../README.md) | [简体中文](./README_zh_CN.md) | [繁體中文](./README_zh_TW.md) | [हिन्दी](./README_hi.md) | [Español](./README_es.md) | [Français](./README_fr.md) | [العربية](./README_ar.md) | [বাংলা](./README_bn.md) | [Português](./README_pt.md) | [Русский](./README_ru.md) | [اردو](./README_ur.md) | [Bahasa Indonesia](./README_id.md) | [Deutsch](./README_de.md) | [日本語](./README_ja.md) | [Türkçe](./README_tr.md) | [한국어](./README_ko.md) | [Italiano](./README_it.md) | [Nederlands](./README_nl.md) | Svenska | [Čeština](./README_cs.md) | [Ελληνικά](./README_el.md) | [Magyar](./README_hu.md) | [עברית](./README_he.md) | [Norsk](./README_no.md) | [Dansk](./README_da.md) | [Suomi](./README_fi.md) | [Slovenčina](./README_sk.md) | [Slovenščina](./README_sl.md) | [Íslenska](./README_is.md)

---

## 📑 Innehållsförteckning
- [🌍 Språkstöd](#-språkstöd)
- [🚀 Huvudfunktioner & Användning](#-huvudfunktioner--användning)
- [⚙️ Konfiguration & Loggar](#️-konfiguration--loggar)
- [🖼️ Skärmbilder](#️-skärmbilder)
- [🎬 Demonstration](#-demonstration)
- [💻 Systemkrav](#-systemkrav)
- [📦 Installationsguide](#-installationsguide)
- [🛠️ Plattform & Prestanda](#️-plattform--prestanda)
- [📄 Licens](#-licens)

---

## 🌍 Språkstöd

Engelska, Kinesiska (Förenklad), Kinesiska (Traditionell), Hindi, Spanska, Franska, Arabiska, Bengaliska, Portugisiska, Ryska, Urdu, Indonesiska, Tyska, Japanska, Turkiska, Koreanska, Italienska, Nederländska, Svenska, Tjeckiska, Grekiska, Ungerska, Hebreiska, Norska, Danska, Finska, Slovakiska, Slovenska, Isländska

<p align="left">
  <img src="../assets/flags/us.svg" width="32" title="Engelska" alt="Engelska" />
  <img src="../assets/flags/cn.svg" width="32" title="Kinesiska (Förenklad)" alt="Kinesiska (Förenklad)" />
  <img src="../assets/flags/tw.svg" width="32" title="Kinesiska (Traditionell)" alt="Kinesiska (Traditionell)" />
  <img src="../assets/flags/in.svg" width="32" title="Hindi" alt="Hindi" />
  <img src="../assets/flags/es.svg" width="32" title="Spanska" alt="Spanska" />
  <img src="../assets/flags/fr.svg" width="32" title="Franska" alt="Franska" />
  <img src="../assets/flags/sa.svg" width="32" title="Arabiska" alt="Arabiska" />
  <img src="../assets/flags/bd.svg" width="32" title="Bengaliska" alt="Bengaliska" />
  <img src="../assets/flags/pt.svg" width="32" title="Portugisiska" alt="Portugisiska" />
  <img src="../assets/flags/ru.svg" width="32" title="Ryska" alt="Ryska" />
  <img src="../assets/flags/pk.svg" width="32" title="Urdu" alt="Urdu" />
  <img src="../assets/flags/id.svg" width="32" title="Indonesiska" alt="Indonesiska" />
  <img src="../assets/flags/de.svg" width="32" title="Tyska" alt="Tyska" />
  <img src="../assets/flags/jp.svg" width="32" title="Japanska" alt="Japanska" />
  <img src="../assets/flags/tr.svg" width="32" title="Turkiska" alt="Turkiska" />
  <img src="../assets/flags/kr.svg" width="32" title="Koreanska" alt="Koreanska" />
  <img src="../assets/flags/it.svg" width="32" title="Italienska" alt="Italienska" />
  <img src="../assets/flags/nl.svg" width="32" title="Nederländska" alt="Nederländska" />
  <img src="../assets/flags/se.svg" width="32" title="Svenska" alt="Svenska" />
  <img src="../assets/flags/cz.svg" width="32" title="Tjeckiska" alt="Tjeckiska" />
  <img src="../assets/flags/gr.svg" width="32" title="Grekiska" alt="Grekiska" />
  <img src="../assets/flags/hu.svg" width="32" title="Ungerska" alt="Ungerska" />
  <img src="../assets/flags/il.svg" width="32" title="Hebreiska" alt="Hebreiska" />
  <img src="../assets/flags/no.svg" width="32" title="Norska" alt="Norska" />
  <img src="../assets/flags/dk.svg" width="32" title="Danska" alt="Danska" />
  <img src="../assets/flags/fi.svg" width="32" title="Finska" alt="Finska" />
  <img src="../assets/flags/sk.svg" width="32" title="Slovakiska" alt="Slovakiska" />
  <img src="../assets/flags/si.svg" width="32" title="Slovenska" alt="Slovenska" />
  <img src="../assets/flags/is.svg" width="32" title="Isländska" alt="Isländska" />
</p>


## 🚀 Huvudfunktioner & Användning

- **Modernt nativt UI**: Intuitivt GUI med stöd för mörkt/ljust läge, mjuka animationer och högpresterande rendering via **Skia**.
- **Systemfältsintegration (Tray)**: Fullt stöd för minimering till systemfältet (~10 MB RAM-användning), dubbelklicka för att växla mellan fönster och en funktionell högerklicksmeny.
- **Intelligent start**: Konfigurera panelen för att starta med Windows, minimera till systemfältet (tyst läge med `/silent`) och stänga av distributioner automatiskt vid avslut.
- **Omfattande instanskontroll**: Starta, stoppa, avsluta och avregistrera med ett klick. Statusövervakning i realtid och detaljerad information om diskanvändning och filplatser.
- **Hantering av distributioner**: Ställ in som standard, migrering (flytta VHDX till andra enheter) och exportera/klona till `.tar` eller `.tar.gz`-arkiv.
- **Snabb integration**: Starta omedelbart i Terminal, VS Code eller Utforskaren med anpassningsbara arbetskataloger och startskriptshooks.
- **Smart installation**: Installera från Microsoft Store, GitHub eller lokala filer (RootFS/VHDX). Inkluderar en inbyggd RootFS-nedladdningsassistent.
- **Säkerhet**: Mutex-lås för säkra samtidiga migrerings-/backup-operationer och automatisk Appx-rensning vid borttagning.
- **Minimalt minnesavtryck**: Högt optimerad för effektivitet. Tyst start (systemfältet) använder endast **~10 MB** RAM. Användning i fönsterläge varierar beroende på tecknens komplexitet: **~18 MB** för standardspråk och **~38 MB** för språk med stora teckenuppsättningar (kinesiska, japanska, koreanska).


## ⚙️ Konfiguration & Loggar

All konfiguration hanteras via inställningsvyn:

- Välj standardkatalog för installation av nya WSL-instanser.
- Konfigurera loggkatalog och loggnivå (Error / Warn / Info / Debug / Trace).
- Välj UI-språk eller låt det följa systemspråket.
- Växla mörkt läge och välj om appen ska stänga av WSL automatiskt efter operationer.
- Konfigurera hur ofta appen söker efter uppdateringar (dagligen, veckovis, varannan vecka, månadsvis).
- Aktivera automatisk start vid systemstart (med automatisk reparation av sökvägar).
- Ställ in appen att minimera till systemfältet vid start.
- Konfigurera stängningsknappen att minimera till systemfältet istället för att avsluta.

Loggfiler skrivs till den konfigurerade loggkatalogen och kan bifogas vid felrapportering.


## 🖼️ Skärmbilder

### Hem (Ljust & Mörkt läge)
<p align="center">
  <img src="../assets/screenshot/home.png" width="48%" />
  <img src="../assets/screenshot/home-dark.png" width="48%" />
</p>

### Lägg till instans & Inställningar
<p align="center">
  <img src="../assets/screenshot/add.png" width="48%" />
  <img src="../assets/screenshot/settings.png" width="48%" />
</p>

### Om & Menyminimering
<p align="center">
  <img src="../assets/screenshot/about.png" width="48%" />
  <img src="../assets/screenshot/collapsed.png" width="48%" />
</p>

## 🎬 Demonstration

Här är en demonstration av WSL Dashboard i arbete:

![WSL Dashboard Demo](../assets/screenshot/demo.gif)



## 💻 Systemkrav

- Windows 10 eller Windows 11 med WSL aktiverat (WSL 2 rekommenderas).
- Minst en WSL-distribution installerad, eller behörighet att installera nya.
- 64-bitars CPU; 4 GB RAM eller mer rekommenderas för smidig användning av flera distributioner.

## 📦 Installationsguide

### Alternativ 1: Ladda ner färdigkompilerad binär

Det enklaste sättet att komma igång är att använda den förkompilerade versionen:

1. Gå till sidan för [GitHub Releases](https://github.com/owu/wsl-dashboard/releases).
2. Ladda ner den senaste körbara filen `wsldashboard` för Windows.
3. Packa upp (om den är i ett arkiv) och kör `wsldashboard.exe`.

Ingen installation krävs; appen är en enda portabel binärfil.

### Alternativ 2: Bygg från källkod

Se till att du har Rust-verktygskedjan (Rust 1.92+ eller nyare) installerad.

1. Klona arkivet:

   ```powershell
   git clone https://github.com/owu/wsl-dashboard.git
   cd wsl-dashboard
   ```

2. Bygg och kör:

   - För utveckling:

     ```powershell
     cargo run
     ```
   - Optimerad release-build med byggskriptet:

     > Byggskriptet kräver verktygskedjan `x86_64-pc-windows-msvc`.

     ```powershell
     .\build\scripts\build.ps1
     ```


## 🛠️ Plattform & Prestanda

- **Kärna**: Implementerad i Rust för minnessäkerhet och abstraktioner utan extra kostnad.
- **UI-ramverk**: Slint med högpresterande **Skia**-renderingsmotor.
- **Asynkron körtid**: Tokio för icke-blockerande systemkommandon och I/O.
- **Prestandahöjdpunkter**:
  - **Responsivitet**: Nästan omedelbar start och statusövervakning av WSL i realtid.
  - **Effektivitet**: Ultralåg resursförbrukning (se [Huvudfunktioner](#-huvudfunktioner--användning) för detaljer).
  - **Portabilitet**: Optimerad release-build skapar en enda kompakt körbar fil.



## 📄 Licens

Detta projekt är licensierat under GPL-3.0 – se filen [LICENSE](../LICENSE) för detaljer.

---

Byggd med ❤️ för WSL-communityn.
