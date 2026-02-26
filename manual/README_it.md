# WSL Dashboard

<p align="center">
  <img src="../assets/logo/logo.png" width="128" height="128" alt="WSL Dashboard Logo" />
</p>

Una dashboard moderna, performante e leggera per la gestione delle istanze WSL (Windows Subsystem for Linux). Realizzata con Rust e Slint per un'esperienza nativa di alto livello.

---

<p align="left">
  <a href="https://www.rust-lang.org"><img src="https://img.shields.io/badge/rust-v1.92+-orange.svg" alt="Rust" /></a>
  <a href="https://slint.dev"><img src="https://img.shields.io/badge/UI-Slint-blue.svg" alt="Slint" /></a>
  <a href="https://tokio.rs"><img src="https://img.shields.io/badge/Async-Tokio-000000.svg" alt="Tokio" /></a>
  <a href="https://github.com/microsoft/windows-rs"><img src="https://img.shields.io/badge/OS-Windows-0078D6.svg" alt="Windows" /></a>
  <a href="../LICENSE"><img src="https://img.shields.io/badge/license-GPL--3.0-blue.svg" alt="License" /></a>
</p>

I18N :  [English](../README.md) | [简体中文](./README_zh_CN.md) | [繁體中文](./README_zh_TW.md) | [हिन्दी](./README_hi.md) | [Español](./README_es.md) | [Français](./README_fr.md) | [العربية](./README_ar.md) | [বাংলা](./README_bn.md) | [Português](./README_pt.md) | [Русский](./README_ru.md) | [اردو](./README_ur.md) | [Bahasa Indonesia](./README_id.md) | [Deutsch](./README_de.md) | [日本語](./README_ja.md) | [Türkçe](./README_tr.md) | [한국어](./README_ko.md) | Italiano | [Nederlands](./README_nl.md) | [Svenska](./README_sv.md) | [Čeština](./README_cs.md) | [Ελληνικά](./README_el.md) | [Magyar](./README_hu.md) | [עברית](./README_he.md) | [Norsk](./README_no.md) | [Dansk](./README_da.md) | [Suomi](./README_fi.md) | [Slovenčina](./README_sk.md) | [Slovenščina](./README_sl.md) | [Íslenska](./README_is.md)

---

## 📑 Sommario
- [🌍 Lingue Supportate](#-lingue-supportate)
- [🚀 Funzionalità Principali e Uso](#-funzionalità-principali-e-uso)
- [⚙️ Configurazione e Log](#️-configurazione-e-log)
- [🖼️ Screenshot](#️-screenshot)
- [🎬 Demo di Funzionamento](#-demo-di-funzionamento)
- [💻 Requisiti di Sistema](#-requisiti-di-sistema)
- [📦 Guida all'Installazione](#-guida-allinstallazione)
- [🛠️ Stack Tecnologico e Performance](#️-stack-tecnologico-e-performance)
- [📄 Licenza](#-licenza)

---

## 🌍 Lingue Supportate

Inglese, Cinese (Semplificato), Cinese (Tradizionale), Hindi, Spagnolo, Francese, Arabo, Bengalese, Portoghese, Russo, Urdu, Indonesiano, Tedesco, Giapponese, Turco, Coreano, Italiano, Olandese, Svedese, Ceco, Greco, Ungherese, Ebraico, Norvegese, Danese, Finlandese, Slovacco, Sloveno, Islandese

<p align="left">
  <img src="../assets/flags/us.svg" width="32" title="Inglese" alt="Inglese" />
  <img src="../assets/flags/cn.svg" width="32" title="Cinese (Semplificato)" alt="Cinese (Semplificato)" />
  <img src="../assets/flags/tw.svg" width="32" title="Cinese (Tradizionale)" alt="Cinese (Tradizionale)" />
  <img src="../assets/flags/in.svg" width="32" title="Hindi" alt="Hindi" />
  <img src="../assets/flags/es.svg" width="32" title="Spagnolo" alt="Spagnolo" />
  <img src="../assets/flags/fr.svg" width="32" title="Francese" alt="Francese" />
  <img src="../assets/flags/sa.svg" width="32" title="Arabo" alt="Arabo" />
  <img src="../assets/flags/bd.svg" width="32" title="Bengalese" alt="Bengalese" />
  <img src="../assets/flags/pt.svg" width="32" title="Portoghese" alt="Portoghese" />
  <img src="../assets/flags/ru.svg" width="32" title="Russo" alt="Russo" />
  <img src="../assets/flags/pk.svg" width="32" title="Urdu" alt="Urdu" />
  <img src="../assets/flags/id.svg" width="32" title="Indonesiano" alt="Indonesiano" />
  <img src="../assets/flags/de.svg" width="32" title="Tedesco" alt="Tedesco" />
  <img src="../assets/flags/jp.svg" width="32" title="Giapponese" alt="Giapponese" />
  <img src="../assets/flags/tr.svg" width="32" title="Turco" alt="Turco" />
  <img src="../assets/flags/kr.svg" width="32" title="Coreano" alt="Coreano" />
  <img src="../assets/flags/it.svg" width="32" title="Italiano" alt="Italiano" />
  <img src="../assets/flags/nl.svg" width="32" title="Olandese" alt="Olandese" />
  <img src="../assets/flags/se.svg" width="32" title="Svedese" alt="Svedese" />
  <img src="../assets/flags/cz.svg" width="32" title="Ceco" alt="Ceco" />
  <img src="../assets/flags/gr.svg" width="32" title="Greco" alt="Greco" />
  <img src="../assets/flags/hu.svg" width="32" title="Ungherese" alt="Ungherese" />
  <img src="../assets/flags/il.svg" width="32" title="Ebraico" alt="Ebraico" />
  <img src="../assets/flags/no.svg" width="32" title="Norvegese" alt="Norvegese" />
  <img src="../assets/flags/dk.svg" width="32" title="Danese" alt="Danese" />
  <img src="../assets/flags/fi.svg" width="32" title="Finlandese" alt="Finlandese" />
  <img src="../assets/flags/sk.svg" width="32" title="Slovacco" alt="Slovacco" />
  <img src="../assets/flags/si.svg" width="32" title="Sloveno" alt="Sloveno" />
  <img src="../assets/flags/is.svg" width="32" title="Islandese" alt="Islandese" />
</p>


## 🚀 Funzionalità Principali e Uso

- **Interfaccia Nativa Moderna**: GUI intuitiva con supporto alla modalità chiara/scura, animazioni fluide e rendering ad alte prestazioni basato su **Skia**.
- **Integrazione System Tray**: Supporto completo per la riduzione nell'area di notifica (~10MB di RAM), doppio clic per mostrare/nascondere e menu contestuale funzionale.
- **Avvio Intelligente**: Configura la dashboard per avviarsi con Windows, ridursi nel tray (modalità silenziosa con `/silent`) e arrestare automaticamente le distribuzioni alla chiusura.
- **Controllo Completo Istanze**: Avvio, stop, terminazione e rimozione in un clic. Monitoraggio dello stato in tempo reale e approfondimenti su utilizzo del disco e posizione dei file.
- **Gestione Distribuzioni**: Imposta come predefinita, migrazione (sposta il VHDX su altre unità) ed esportazione/clonazione in formato `.tar` o `.tar.gz`.
- **Integrazione Rapida**: Lancio istantaneo di Terminale, VS Code o Esplora File con directory di lavoro personalizzabili e hook per script di avvio.
- **Installazione Intelligente**: Installa dal Microsoft Store, GitHub o file locali (RootFS/VHDX). Assistente al download RootFS integrato.
- **Sicurezza Globale**: Blocchi mutex per operazioni di migrazione/backup simultanee sicure e pulizia automatica di Appx alla rimozione.
- **Impronta di Memoria Bassissima**: Estremamente ottimizzato. L'avvio silenzioso (tray) utilizza solo **~10MB** di RAM. L'uso in modalità finestra varia in base alla complessità dei caratteri: **~18MB** per lingue standard e **~38MB** per lingue con set di caratteri estesi (Cinese, Giapponese, Coreano).


## ⚙️ Configurazione e Log

Tutta la configurazione è gestita tramite la vista Impostazioni:

- Scegli la directory di installazione predefinita per le nuove istanze WSL.
- Configura la directory dei log e il livello di log (Error / Warn / Info / Debug / Trace).
- Scegli la lingua dell'interfaccia o segui la lingua di sistema.
- Attiva o disattiva la modalità scura e consenti all'app di arrestare automaticamente WSL dopo le operazioni.
- Configura la frequenza con cui l'app controlla gli aggiornamenti (giornaliera, settimanale, bisettimanale, mensile).
- Abilita l'avvio automatico all'accesso del sistema (con riparazione automatica dei percorsi).
- Imposta l'app per ridursi nel tray all'avvio.
- Configura il pulsante di chiusura per ridurre nel tray invece di uscire dal programma.

I file di log vengono scritti nella directory configurata e possono essere allegati quando si segnalano problemi.


## 🖼️ Screenshot

### Home (Modalità Scura e Chiara)
<p align="center">
  <img src="../assets/screenshot/home.png" width="48%" />
  <img src="../assets/screenshot/home-dark.png" width="48%" />
</p>

### Aggiungi Istanza e Impostazioni
<p align="center">
  <img src="../assets/screenshot/add.png" width="48%" />
  <img src="../assets/screenshot/settings.png" width="48%" />
</p>

### Informazioni e menu compresso
<p align="center">
  <img src="../assets/screenshot/about.png" width="48%" />
  <img src="../assets/screenshot/collapsed.png" width="48%" />
</p>

## 🎬 Demo di Funzionamento

Di seguito è riportata una dimostrazione di WSL Dashboard in azione:

![Demo WSL Dashboard](../assets/screenshot/demo.gif)



## 💻 Requisiti di Sistema

- Windows 10 o Windows 11 con WSL abilitato (consigliato WSL 2).
- Almeno una distribuzione WSL installata, o il permesso di installarne di nuove.
- CPU a 64 bit; consigliati 4 GB di RAM o più per un utilizzo fluido.

## 📦 Guida all'Installazione

### Opzione 1: Scarica il binario precompilato

Il modo più semplice per iniziare è utilizzare la versione precompilata:

1. Vai alla pagina [GitHub Releases](https://github.com/owu/wsl-dashboard/releases).
2. Scarica l'ultimo eseguibile `wsldashboard` per Windows.
3. Estrai (se necessario) ed esegui `wsldashboard.exe`.

Non è richiesto alcun programma di installazione; l'app è un unico binario portatile.

### Opzione 2: Compila dai sorgenti

Assicurati di avere installato la toolchain Rust (Rust 1.92+ o successivo).

1. Clona il repository:

   ```powershell
   git clone https://github.com/owu/wsl-dashboard.git
   cd wsl-dashboard
   ```

2. Compila ed esegui:

   - Per lo sviluppo:

     ```powershell
     cargo run
     ```
   - Crea una build di rilascio ottimizzata tramite lo script:

     > Lo script di build richiede la toolchain `x86_64-pc-windows-msvc`.

     ```powershell
     .\build\scripts\build.ps1
     ```


## 🛠️ Stack Tecnologico e Performance

- **Core**: Implementato in Rust per sicurezza della memoria e astrazioni a costo zero.
- **UI Framework**: Slint con backend di rendering **Skia** ad alte prestazioni.
- **Async Runtime**: Tokio per comandi di sistema e I/O non bloccanti.
- **Performance Highlights**:
  - **Reattività**: Avvio quasi istantaneo e monitoraggio dello stato WSL in tempo reale.
  - **Efficienza**: Utilizzo risorse bassissimo (dettagli in [Funzionalità Principali](#-funzionalità-principali-e-uso)).
  - **Portabilità**: La build ottimizzata produce un unico eseguibile compatto.



## 📄 Licenza

Questo progetto è rilasciato sotto licenza GPL-3.0 – consulta il file [LICENSE](../LICENSE) per i dettagli.

---

Built with ❤️ for the WSL Community.
