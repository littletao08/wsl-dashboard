# WSL Dashboard

<p align="center">
  <img src="../assets/logo/logo.png" width="128" height="128" alt="WSL Dashboard Logo" />
</p>

Moderni, suorituskykyinen ja kevyt hallintapaneeli WSL (Windows Subsystem for Linux) -instanssien hallintaan. Rakennettu Rustilla ja Slintillä ensiluokkaista natiivikokemusta varten.

---

<p align="left">
  <a href="https://www.rust-lang.org"><img src="https://img.shields.io/badge/rust-v1.92+-orange.svg" alt="Rust" /></a>
  <a href="https://slint.dev"><img src="https://img.shields.io/badge/UI-Slint-blue.svg" alt="Slint" /></a>
  <a href="https://tokio.rs"><img src="https://img.shields.io/badge/Async-Tokio-000000.svg" alt="Tokio" /></a>
  <a href="https://github.com/microsoft/windows-rs"><img src="https://img.shields.io/badge/OS-Windows-0078D6.svg" alt="Windows" /></a>
  <a href="../LICENSE"><img src="https://img.shields.io/badge/license-GPL--3.0-blue.svg" alt="Lisenssi" /></a>
</p>

I18N :  [English](../README.md) | [简体中文](./README_zh_CN.md) | [繁體中文](./README_zh_TW.md) | [हिन्दी](./README_hi.md) | [Español](./README_es.md) | [Français](./README_fr.md) | [العربية](./README_ar.md) | [বাংলা](./README_bn.md) | [Português](./README_pt.md) | [Русский](./README_ru.md) | [اردو](./README_ur.md) | [Bahasa Indonesia](./README_id.md) | [Deutsch](./README_de.md) | [日本語](./README_ja.md) | [Türkçe](./README_tr.md) | [한국어](./README_ko.md) | [Italiano](./README_it.md) | [Nederlands](./README_nl.md) | [Svenska](./README_sv.md) | [Čeština](./README_cs.md) | [Ελληνικά](./README_el.md) | [Magyar](./README_hu.md) | [עברית](./README_he.md) | [Norsk](./README_no.md) | [Dansk](./README_da.md) | Suomi | [Slovenčina](./README_sk.md) | [Slovenščina](./README_sl.md) | [Íslenska](./README_is.md)

---

## 📑 Sisällysluettelo
- [🌍 Kielituki](#-kielituki)
- [🚀 Tärkeimmät ominaisuudet ja käyttö](#-tärkeimmät-ominaisuudet-ja-käyttö)
- [⚙️ Asetukset ja lokit](#️-asetukset-ja-lokit)
- [🖼️ Kuvakaappaukset](#️-kuvakaappaukset)
- [🎬 Demonstraatio](#-demonstraatio)
- [💻 Järjestelmävaatimukset](#-järjestelmävaatimukset)
- [📦 Asennusopas](#-asennusopas)
- [🛠️ Teknologiapina ja suorituskyky](#️-teknologiapina-ja-suorituskyky)
- [📄 Lisenssi](#-lisenssi)

---

## 🌍 Kielituki

Englanti, Kiina (Yksinkertaistettu), Kiina (Perinteinen), Hindi, Espanja, Ranska, Arabia, Bengali, Portugali, Venäjä, Urdu, Indonesia, Saksa, Japani, Turkki, Korea, Italia, Hollanti, Ruotsi, Tšekki, Kreikka, Unkari, Heprea, Norja, Tanska, Suomi, Slovakki, Sloveeni, Islanti

<p align="left">
  <img src="../assets/flags/us.svg" width="32" title="Englanti" alt="Englanti" />
  <img src="../assets/flags/cn.svg" width="32" title="Kiina (Yksinkertaistettu)" alt="Kiina (Yksinkertaistettu)" />
  <img src="../assets/flags/tw.svg" width="32" title="Kiina (Perinteinen)" alt="Kiina (Perinteinen)" />
  <img src="../assets/flags/in.svg" width="32" title="Hindi" alt="Hindi" />
  <img src="../assets/flags/es.svg" width="32" title="Espanja" alt="Espanja" />
  <img src="../assets/flags/fr.svg" width="32" title="Ranska" alt="Ranska" />
  <img src="../assets/flags/sa.svg" width="32" title="Arabia" alt="Arabia" />
  <img src="../assets/flags/bd.svg" width="32" title="Bengali" alt="Bengali" />
  <img src="../assets/flags/pt.svg" width="32" title="Portugali" alt="Portugali" />
  <img src="../assets/flags/ru.svg" width="32" title="Venäjä" alt="Venäjä" />
  <img src="../assets/flags/pk.svg" width="32" title="Urdu" alt="Urdu" />
  <img src="../assets/flags/id.svg" width="32" title="Indonesia" alt="Indonesia" />
  <img src="../assets/flags/de.svg" width="32" title="Saksa" alt="Saksa" />
  <img src="../assets/flags/jp.svg" width="32" title="Japani" alt="Japani" />
  <img src="../assets/flags/tr.svg" width="32" title="Turkki" alt="Turkki" />
  <img src="../assets/flags/kr.svg" width="32" title="Korea" alt="Korea" />
  <img src="../assets/flags/it.svg" width="32" title="Italia" alt="Italia" />
  <img src="../assets/flags/nl.svg" width="32" title="Hollanti" alt="Hollanti" />
  <img src="../assets/flags/se.svg" width="32" title="Ruotsi" alt="Ruotsi" />
  <img src="../assets/flags/cz.svg" width="32" title="Tšekki" alt="Tšekki" />
  <img src="../assets/flags/gr.svg" width="32" title="Kreikka" alt="Kreikka" />
  <img src="../assets/flags/hu.svg" width="32" title="Unkari" alt="Unkari" />
  <img src="../assets/flags/il.svg" width="32" title="Heprea" alt="Heprea" />
  <img src="../assets/flags/no.svg" width="32" title="Norja" alt="Norja" />
  <img src="../assets/flags/dk.svg" width="32" title="Tanska" alt="Tanska" />
  <img src="../assets/flags/fi.svg" width="32" title="Suomi" alt="Suomi" />
  <img src="../assets/flags/sk.svg" width="32" title="Slovakki" alt="Slovakki" />
  <img src="../assets/flags/si.svg" width="32" title="Sloveeni" alt="Sloveeni" />
  <img src="../assets/flags/is.svg" width="32" title="Islanti" alt="Islanti" />
</p>


## 🚀 Tärkeimmät ominaisuudet ja käyttö

- **Moderni natiivi käyttöliittymä**: Intuitiivinen GUI, jossa on tumma/vaalea tila -tuki, sulavat animaatiot ja korkean suorituskyvyn rendering, jota tukee **Skia**.
- **Järjestelmäilmoitusalueen integraatio (Tray)**: Täysi tuki ilmoitusalueelle pienentämiselle (~10 MB RAM-muistin käyttö), kaksoisnapsautus tilan vaihtamiseksi ja toimiva oikean painikkeen valikko.
- **Älykäs käynnistys**: Määritä hallintapaneeli käynnistymään Windowsin mukana, pienentymään ilmoitusalueelle (hiljainen tila `/silent`-parametrilla) ja sammuttamaan distributiot automaattisesti poistuttaessa.
- **Kattava instanssien hallinta**: Käynnistä, pysäytä, lopeta ja poista rekisteröinti yhdellä napsautuksella. Reaaliaikainen tilan seuranta ja yksityiskohtaiset tiedot levynkäytöstä ja tiedostojen sijainneista.
- **Distributioiden hallinta**: Aseta oletukseksi, siirrä (VHDX siirto muille asemille) ja vie/kloonaa `.tar` tai `.tar.gz` -arkistoihin.
- **Nopea integraatio**: Välitön käynnistys terminaaliin, VS Codeen tai tiedostonhallintaan muokattavilla työhakemistoilla ja käynnistysskriptien tuella.
- **Älykäs asennus**: Asenna Microsoft Storesta, GitHubista tai paikallisista tiedostoista (RootFS/VHDX). Sisältää sisäänrakennetun RootFS-latausapulaisen.
- **Globaali turvallisuus**: Mutex-lukot turvallisia rinnakkaisia siirto-/varmuuskopiointioperaatioita varten ja automaattinen Appx-puhdistus poiston yhteydessä.
- **Erittäin pieni muistijälki**: Optimoitu tehokkuutta varten. Hiljainen käynnistys (ilmoitusalueelle) käyttää vain **n. 10 MB** RAM-muistia. Ikkunatilassa käyttö vaihtelee kirjasinten monimutkaisuuden mukaan: **n. 18 MB** peruskielille ja **n. 38 MB** kielille, joissa on laajat merkistöt (kiina, japani, korea).


## ⚙️ Asetukset ja lokit

Kaikki asetukset hallitaan Asetukset-näkymän kautta:

- Valitse oletusasennushakemisto uusille WSL-instansseille.
- Määritä lokihakemisto ja lokitaso (Error / Warn / Info / Debug / Trace).
- Valitse käyttöliittymän kieli tai anna sen seurata järjestelmän kieltä.
- Vaihda tumma tila ja valitse, voiko sovellus sammuttaa WSL:n automaattisesti operaatioiden jälkeen.
- Määritä, kuinka usein sovellus tarkistaa päivitykset (päivittäin, viikoittain, kahden viikon välein, kuukausittain).
- Ota käyttöön automaattinen käynnistys järjestelmän käynnistyessä (automaattisella polun korjauksella).
- Aseta sovellus pienentymään ilmoitusalueelle käynnistettäessä.
- Määritä sulkemispainike pienentämään sovellus ilmoitusalueelle sulkemisen sijaan.

Lokitiedostot kirjoitetaan määritettyyn lokihakemistoon ja ne voidaan liittää mukaan ongelmista ilmoitettaessa.


## 🖼️ Kuvakaappaukset

### Koti (Vaalea ja tumma tila)
<p align="center">
  <img src="../assets/screenshot/home.png" width="48%" />
  <img src="../assets/screenshot/home-dark.png" width="48%" />
</p>

### Lisää instanssi ja asetukset
<p align="center">
  <img src="../assets/screenshot/add.png" width="48%" />
  <img src="../assets/screenshot/settings.png" width="48%" />
</p>

### Tietoja ja pienennetty valikko
<p align="center">
  <img src="../assets/screenshot/about.png" width="48%" />
  <img src="../assets/screenshot/collapsed.png" width="48%" />
</p>

## 🎬 Demonstraatio

Alla on demonstraatio WSL Dashboardista toiminnassa:

![WSL Dashboard Demo](../assets/screenshot/demo.gif)



## 💻 Järjestelmävaatimukset

- Windows 10 tai Windows 11, jossa WSL on käytössä (WSL 2 suositeltu).
- Vähintään yksi asennettu WSL-distributio tai oikeus asentaa uusia.
- 64-bittinen CPU; suosittelemme vähintään 4 Gt RAM-muistia useiden distributioiden sujuvaan käyttöön.

## 📦 Asennusopas

### Vaihtoehto 1: Lataa valmiiksi koottu binääri

Helpoin tapa aloittaa on käyttää valmiiksi käännettyä julkaisua:

1. Siirry [GitHub Releases](https://github.com/owu/wsl-dashboard/releases) -sivulle.
2. Lataa uusin Windows-yhteensopiva `wsldashboard`-suoritustiedosto.
3. Pura (jos pakattu) ja suorita `wsldashboard.exe`.

Asennusohjelmaa ei tarvita; sovellus on yksi kannettava binääritiedosto.

### Vaihtoehto 2: Kokoa lähdekoodista

Varmista, että sinulla on Rust-työkalukehys (Rust 1.92+ tai uudempi) asennettuna.

1. Kloonaa repositorio:

   ```powershell
   git clone https://github.com/owu/wsl-dashboard.git
   cd wsl-dashboard
   ```

2. Kokoa ja suorita:

   - Kehityskäyttöön:

     ```powershell
     cargo run
     ```
   - Optimoitu julkaisuversio rakennusskriptin avulla:

     > Rakennusskripti vaatii `x86_64-pc-windows-msvc` -työkalukehyksen.

     ```powershell
     .\build\scripts\build.ps1
     ```


## 🛠️ Teknologiapina ja suorituskyky

- **Ydin**: Toteutettu Rustilla muistiturvallisuuden ja nollakustannusabstraktioiden saavuttamiseksi.
- **UI-kehys**: Slint, jossa on korkean suorituskyvyn **Skia**-renderöintitausta.
- **Async Runtime**: Tokio ei-blokkaavia järjestelmäkomentoja ja I/O:ta varten.
- **Suorituskyvyn kohokohdat**:
  - **Vasteaika**: Lähes välitön käynnistys ja reaaliaikainen WSL-tilan seuranta.
  - **Tehokkuus**: Erittäin pieni resurssien käyttö (katso lisätietoja kohdasta [Tärkeimmät ominaisuudet](#-tärkeimmät-ominaisuudet-ja-käyttö)).
  - **Siirrettävyys**: Optimoitu julkaisuversio tuottaa yhden tiiviin suoritustiedoston.



## 📄 Lisenssi

Tämä projekti on lisensoitu GPL-3.0-lisenssillä – katso tarkemmat tiedot [LICENSE](../LICENSE)-tiedostosta.

---

Rakennettu ❤️:lla WSL-yhteisölle.
