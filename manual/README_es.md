# WSL Dashboard

<p align="center">
  <img src="../assets/logo/logo.png" width="128" height="128" alt="WSL Dashboard Logo" />
</p>

Un panel de gestión de instancias de WSL (Windows Subsystem for Linux) moderno, de alto rendimiento y ligero. Construido con Rust y Slint para una experiencia nativa premium.

---

<p align="left">
  <a href="https://www.rust-lang.org"><img src="https://img.shields.io/badge/rust-v1.92+-orange.svg" alt="Rust" /></a>
  <a href="https://slint.dev"><img src="https://img.shields.io/badge/UI-Slint-blue.svg" alt="Slint" /></a>
  <a href="https://tokio.rs"><img src="https://img.shields.io/badge/Async-Tokio-000000.svg" alt="Tokio" /></a>
  <a href="https://github.com/microsoft/windows-rs"><img src="https://img.shields.io/badge/OS-Windows-0078D6.svg" alt="Windows" /></a>
  <a href="../LICENSE"><img src="https://img.shields.io/badge/license-GPL--3.0-blue.svg" alt="License" /></a>
</p>

I18N :  [English](../README.md) | [简体中文](./README_zh_CN.md) | [繁體中文](./README_zh_TW.md) | [हिन्दी](./README_hi.md) | Español | [Français](./README_fr.md) | [العربية](./README_ar.md) | [বাংলা](./README_bn.md) | [Português](./README_pt.md) | [Русский](./README_ru.md) | [اردو](./README_ur.md) | [Bahasa Indonesia](./README_id.md) | [Deutsch](./README_de.md) | [日本語](./README_ja.md) | [Türkçe](./README_tr.md) | [한국어](./README_ko.md) | [Italiano](./README_it.md) | [Nederlands](./README_nl.md) | [Svenska](./README_sv.md) | [Čeština](./README_cs.md) | [Ελληνικά](./README_el.md) | [Magyar](./README_hu.md) | [עברית](./README_he.md) | [Norsk](./README_no.md) | [Dansk](./README_da.md) | [Suomi](./README_fi.md) | [Slovenčina](./README_sk.md) | [Slovenščina](./README_sl.md) | [Íslenska](./README_is.md)

---

## 📑 Tabla de Contenidos
- [🌍 Idiomas Soportados](#-idiomas-soportados)
- [🚀 Características Clave y Uso](#-características-clave-y-uso)
- [⚙️ Configuración y Registros](#️-configuración-y-registros)
- [🖼️ Capturas de Pantalla](#️-capturas-de-pantalla)
- [🎬 Demostración](#-demostración)
- [💻 Requisitos del Sistema](#-requisitos-del-sistema)
- [📦 Guía de Instalación](#-guía-de-instalación)
- [🛠️ Tecnologías y Rendimiento](#️-tecnologías-y-rendimiento)
- [📄 Licencia](#-licencia)

---

## 🌍 Idiomas Soportados

Inglés, Chino, Chino, Hindi, Español, Francés, Arabic, Bengalí, Portugués, Ruso, Urdu, Indonesio, Alemán, Japonés, Turco, Korean, Italiano, Dutch, Swedish, Czech, Greek, Hungarian, Hebrew, Norwegian, Danish, Finnish, Slovak, Slovenian, Icelandic

<p align="left">
  <img src="../assets/flags/us.svg" width="32" title="Inglés" alt="Inglés" />
  <img src="../assets/flags/cn.svg" width="32" title="Chino (Simplificado)" alt="Chino (Simplificado)" />
  <img src="../assets/flags/tw.svg" width="32" title="Chino (Tradicional)" alt="Chino (Tradicional)" />
  <img src="../assets/flags/in.svg" width="32" title="Hindi" alt="Hindi" />
  <img src="../assets/flags/es.svg" width="32" title="Español" alt="Español" />
  <img src="../assets/flags/fr.svg" width="32" title="Francés" alt="Francés" />
  <img src="../assets/flags/sa.svg" width="32" title="Árabe" alt="Árabe" />
  <img src="../assets/flags/bd.svg" width="32" title="Bengalí" alt="Bengalí" />
  <img src="../assets/flags/pt.svg" width="32" title="Portugués" alt="Portugués" />
  <img src="../assets/flags/ru.svg" width="32" title="Ruso" alt="Ruso" />
  <img src="../assets/flags/pk.svg" width="32" title="Urdu" alt="Urdu" />
  <img src="../assets/flags/id.svg" width="32" title="Indonesio" alt="Indonesio" />
  <img src="../assets/flags/de.svg" width="32" title="Alemán" alt="Alemán" />
  <img src="../assets/flags/jp.svg" width="32" title="Japonés" alt="Japonés" />
  <img src="../assets/flags/tr.svg" width="32" title="Turco" alt="Turco" />
  <img src="../assets/flags/kr.svg" width="32" title="Coreano" alt="Coreano" />
  <img src="../assets/flags/it.svg" width="32" title="Italiano" alt="Italiano" />
  <img src="../assets/flags/nl.svg" width="32" title="Holandés" alt="Holandés" />
  <img src="../assets/flags/se.svg" width="32" title="Sueco" alt="Sueco" />
  <img src="../assets/flags/cz.svg" width="32" title="Checo" alt="Checo" />
  <img src="../assets/flags/gr.svg" width="32" title="Griego" alt="Griego" />
  <img src="../assets/flags/hu.svg" width="32" title="Húngaro" alt="Húngaro" />
  <img src="../assets/flags/il.svg" width="32" title="Hebreo" alt="Hebreo" />
  <img src="../assets/flags/no.svg" width="32" title="Noruego" alt="Noruego" />
  <img src="../assets/flags/dk.svg" width="32" title="Danés" alt="Danés" />
  <img src="../assets/flags/fi.svg" width="32" title="Finlandés" alt="Finlandés" />
  <img src="../assets/flags/sk.svg" width="32" title="Eslovaco" alt="Eslovaco" />
  <img src="../assets/flags/si.svg" width="32" title="Esloveno" alt="Esloveno" />
  <img src="../assets/flags/is.svg" width="32" title="Islandés" alt="Islandés" />
</p>


## 🚀 Características Clave y Uso

- **Interfaz Nativa Moderna**: GUI intuitiva con soporte para modo claro/oscuro, animaciones fluidas y renderizado de alto rendimiento impulsado por **Skia**.
- **Integración con la Bandeja del Sistema**: Soporte completo para minimizar a la bandeja (~10MB de uso de RAM), doble clic para alternar y un menú contextual funcional.
- **Inicio Inteligente**: Configure el panel para que se inicie con Windows, se minimice en la bandeja (modo silencioso con `/silent`) y apague automáticamente las distribuciones al salir.
- **Control Completo de Instancias**: Inicie, detenga, termine y anule el registro con un clic. Monitorización del estado en tiempo real, información detallada sobre el uso del disco y la ubicación de los archivos.
- **Gestión de Distros**: Establecer como predeterminada, migración (mover VHDX a otras unidades) y exportar/clonar a formatos `.tar` o `.tar.gz`.
- **Integración Rápida**: Lanzamiento instantáneo de Terminal, VS Code o Explorador de archivos con directorios de trabajo personalizables y ganchos de script de inicio.
- **Instalación Inteligente**: Instale desde Microsoft Store, GitHub o archivos locales (RootFS/VHDX). Incluye asistente de descarga RootFS integrado.
- **Seguridad Global**: Bloqueos mutex para operaciones concurrentes seguras de migración/respaldo y limpieza automática de Appx al eliminar.
- **Huella de Memoria Ultra Baja**: Altamente optimizado para la eficiencia. El inicio silencioso (bandeja) usa solo **~10MB** de RAM. El uso en modo ventana varía según la complejidad de la fuente: **~18MB** para idiomas estándar y **~38MB** para idiomas con grandes conjuntos de caracteres (chino, japonés, coreano).


## ⚙️ Configuración y Registros

Toda la configuración se gestiona a través de la vista Configuración:

- Elija el directorio de instalación predeterminado para las nuevas instancias de WSL.
- Configure el directorio de registros y el nivel de registro (Error / Warn / Info / Debug / Trace).
- Seleccione el idioma de la interfaz o deje que siga el idioma del sistema.
- Cambie entre el modo claro y oscuro, y decida si la aplicación puede apagar automáticamente WSL tras las operaciones.
- Configure la frecuencia con la que la aplicación busca actualizaciones (diario, semanal, quincenal, mensual).
- Habilite el inicio automático al arrancar el sistema (con reparación automática de rutas).
- Configure la aplicación para que se minimice en la bandeja al iniciar.
- Configure el botón de cierre para que minimice en la bandeja en lugar de salir del programa.

Los archivos de registro se escriben en el directorio configurado y pueden adjuntarse al informar de problemas.


## 🖼️ Capturas de Pantalla

### Inicio (Modos Oscuro y Claro)
<p align="center">
  <img src="../assets/screenshot/home.png" width="48%" />
  <img src="../assets/screenshot/home-dark.png" width="48%" />
</p>

### Añadir Instancia y Configuración
<p align="center">
  <img src="../assets/screenshot/add.png" width="48%" />
  <img src="../assets/screenshot/settings.png" width="48%" />
</p>

### Acerca de y menú colapsado
<p align="center">
  <img src="../assets/screenshot/about.png" width="48%" />
  <img src="../assets/screenshot/collapsed.png" width="48%" />
</p>

## 🎬 Demostración

A continuación se muestra una demostración de WSL Dashboard en acción:

![Demostración de WSL Dashboard](../assets/screenshot/demo.gif)



## 💻 Requisitos del Sistema

- Windows 10 o Windows 11 con WSL habilitado (se recomienda WSL 2).
- Al menos una distribución de WSL instalada, o permiso para instalar nuevas.
- CPU de 64 bits; se recomienda 4 GB de RAM o más para un uso fluido.

## 📦 Guía de Instalación

### Opción 1: Descargar el binario precompilado

La forma más fácil de empezar es usar el lanzamiento precompilado:

1. Vaya a la página de [GitHub Releases](https://github.com/owu/wsl-dashboard/releases).
2. Descargue el último ejecutable `wsldashboard` para Windows.
3. Extraiga (si está empaquetado) y ejecute `wsldashboard.exe`.

No se requiere instalador; la aplicación es un único binario portátil.

### Opción 2: Construir desde el código fuente

Asegúrese de tener instalado el conjunto de herramientas de Rust (Rust 1.92+ o más reciente).

1. Clone el repositorio:

   ```powershell
   git clone https://github.com/owu/wsl-dashboard.git
   cd wsl-dashboard
   ```

2. Construir y ejecutar:

   - Para desarrollo:

     ```powershell
     cargo run
     ```
   - Crear una construcción de lanzamiento optimizada mediante el script:

     > El script de construcción requiere el conjunto de herramientas `x86_64-pc-windows-msvc`.

     ```powershell
     .\build\scripts\build.ps1
     ```


## 🛠️ Tecnologías y Rendimiento

- **Núcleo**: Implementado en Rust para seguridad de memoria y abstracciones de coste cero.
- **Marco de UI**: Slint con backend de renderizado **Skia** de alto rendimiento.
- **Runtime Async**: Tokio para comandos de sistema y E/S no bloqueantes.
- **Puntos Clave de Rendimiento**:
  - **Capacidad de respuesta**: Inicio casi instantáneo y monitorización del estado de WSL en tiempo real.
  - **Eficiencia**: Uso de recursos ultra bajo (detalles en [Características Clave](#-características-clave-y-uso)).
  - **Portabilidad**: El build optimizado produce un único ejecutable compacto.



## 📄 Licencia

Este proyecto está bajo la GPL-3.0 – vea el archivo [LICENSE](../LICENSE) para más detalles.

---

Built with ❤️ for the WSL Community.
