# 🔐 TruchaSign

<!-- Language Switch -->
**📍 Languages:** [English](#english) | [Español](#español)

---

## English

### 🎯 Overview

**TruchaSign** is an advanced tool for security research and authorized testing that clones Authenticode signature blocks from signed PE (Portable Executable) files and injects them into target binaries. It rewrites the `IMAGE_DIRECTORY_ENTRY_SECURITY` directory and recalculates the `CheckSum` in the Optional Header.

> ⚠️ **Important:** The resulting PE appears signed at the presentation level (Windows Explorer, tools that only read the directory). However, the cryptographic signature chain **is NOT valid** — `truchasignl verify /pa` and `WinVerifyTrust` will reject the binary because the PE digest no longer matches the original CA signature.

### ✨ Features

- ⚡ **Fast & Efficient** — Written in Rust with optimized compilation
- 🔧 **Dual PE Support** — Works with PE32 and PE32+ architectures
- 📊 **Safe Bounds Checking** — Validates all offsets and sizes
- 🎯 **Precise Checksum Calculation** — Uses Microsoft's standard IMAGHELP algorithm
- 🛡️ **Proper Alignment** — Maintains 8-byte alignment requirements for Authenticode

### 📦 Installation

#### Requirements
- Rust 1.70+ (x86_64-pc-windows-msvc target)
- Windows 10/11

#### Build from Source

```powershell
# Install Rust if needed
rustup target add x86_64-pc-windows-msvc

# Clone and build
git clone https://github.com/merovingioops/TruchaSign.git
cd TruchaSign
cargo build --release
```

The optimized binary will be available at `target\release\truchasign.exe`.

#### Using the Setup Script

Alternatively, use the included PowerShell task script for easier setup:

```powershell
# Initialize git repository and create initial commit
.\tasks.ps1 -Task init-repo

# Setup local development environment
.\tasks.ps1 -Task setup-local

# Build the project
.\tasks.ps1 -Task build

# Validate code formatting and quality
.\tasks.ps1 -Task validate

# Clean build artifacts
.\tasks.ps1 -Task clean

# Publish to GitHub
.\tasks.ps1 -Task publish
```

Or simply run `.\tasks.ps1` to see all available options.

### 🚀 Quick Start

```powershell
# Basic syntax
.\target\release\truchasign.exe <signed_source.exe> <target.exe> <output.exe>

# Example: Clone Windows cmd.exe signature to payload
.\target\release\truchasign.exe C:\Windows\System32\cmd.exe .\payload.exe .\payload_signed.exe
```

#### Expected Output

```
[+] signature source: offset=0x12000 size=8456 bytes
[+] injected va=0x4A800 size=8456 checksum=0x0004F3A1
[!] ready -> .\payload_signed.exe
```

### 🔍 How It Works

1. **Parse DOS Header** — Reads `e_lfanew` at offset `0x3C` and validates `MZ` / `PE\0\0`
2. **Detect Architecture** — Identifies PE32 (`0x10B`) or PE32+ (`0x20B`) format
3. **Extract Signature** — Locates Security Data Directory (index 4) and extracts WIN_CERTIFICATE blob
4. **Prepare Target** — Pads PE to 8-byte boundary (Authenticode requirement)
5. **Inject Signature** — Appends signature blob to target PE
6. **Update Headers** — Modifies Security Directory `VirtualAddress` and `Size`
7. **Recalculate Checksum** — Applies Microsoft IMAGHELP algorithm (16-bit sum with carry-fold + file size)

### 🏗️ Architecture

- **`pe_offsets`** — Resolves PE32/PE32+ offsets with `MZ`/`PE` validation
- **`extract_signature`** — Validates and extracts signature blob with bounds checking
- **`calculate_checksum`** — Implements Microsoft's standard checksum algorithm
- **`inject_signature`** — Handles alignment, injection, and header patching

### ✅ Post-Injection Validation

Verify the injection results:

```powershell
# View cloned certificate (expected to show)
sigcheck.exe -i .\payload_signed.exe

# Verify cryptographic chain fails (expected behavior)
Get-AuthenticodeSignature .\payload_signed.exe
truchasignl verify /pa .\payload_signed.exe
```

### ⚠️ Limitations

- ❌ PE/PE32+ only (no other container formats)
- ❌ Cannot reconstruct embedded certificates or catalogs
- ❌ Cryptographic validation via `truchasignl` or `Get-AuthenticodeSignature` will fail (by design)
- ❌ Does not handle corrupted PE structures

### 🤝 Contributing

Contributions are welcome! Please:

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

### ⚖️ Legal Notice

⚠️ **AUTHORIZED USE ONLY**

This tool is designed for:
- 🔬 Authorized security research
- 🎯 Red team exercises (with explicit scope)
- 🏫 CTF (Capture The Flag) competitions
- 🧪 Isolated lab environments

**Unauthorized use against systems is illegal and violates computer fraud laws.** The author assumes no responsibility for misuse or damages resulting from unauthorized access.

### 📄 License

This project is licensed under **MIT OR Apache-2.0** — dual licensed for maximum compatibility and flexibility.

#### Why This License Choice?

For a sensitive security research tool, we've chosen a dual license approach:

- **MIT License**: Simple, permissive, and widely trusted in the open-source community
- **Apache-2.0**: Includes explicit patent grants and detailed legal protections
- **Both**: Allow maximum flexibility while maintaining open-source principles

**⚠️ IMPORTANT**: See the [LICENSE](LICENSE) file for additional terms regarding authorized use. This tool is for security research in authorized contexts only. Unauthorized access to computer systems is illegal.

See the [LICENSE](LICENSE) file for full details.

### 📞 Support & Contact

- 📧 **Email:** [your-email@example.com]
- 🐛 **Report Issues:** [GitHub Issues](https://github.com/merovingioops/TruchaSign/issues)
- 💬 **Discussions:** [GitHub Discussions](https://github.com/merovingioops/TruchaSign/discussions)

---

## Español

### 🎯 Descripción General

**TruchaSign** es una herramienta avanzada para investigación de seguridad y pruebas autorizadas que clona bloques de firma Authenticode desde archivos PE (Portable Executable) firmados e los inyecta en binarios objetivo. Reescribe el directorio `IMAGE_DIRECTORY_ENTRY_SECURITY` y recalcula el `CheckSum` en el Optional Header.

> ⚠️ **Importante:** El PE resultante aparenta estar firmado a nivel de presentación (Explorador de Windows, herramientas que solo leen el directorio). Sin embargo, la cadena de firma criptográfica **NO es válida** — `truchasignl verify /pa` y `WinVerifyTrust` rechazarán el binario porque el digest del PE ya no coincide con la firma original de la CA.

### ✨ Características

- ⚡ **Rápido y Eficiente** — Escrito en Rust con compilación optimizada
- 🔧 **Soporte Dual PE** — Compatible con arquitecturas PE32 y PE32+
- 📊 **Validación Segura** — Verifica todos los offsets y tamaños
- 🎯 **Cálculo Preciso de Checksum** — Utiliza el algoritmo IMAGHELP estándar de Microsoft
- 🛡️ **Alineación Correcta** — Mantiene requisitos de alineación de 8 bytes para Authenticode

### 📦 Instalación

#### Requisitos
- Rust 1.70+ (destino x86_64-pc-windows-msvc)
- Windows 10/11

#### Compilar desde el Código Fuente

```powershell
# Instalar Rust si es necesario
rustup target add x86_64-pc-windows-msvc

# Clonar y compilar
git clone https://github.com/merovingioops/TruchaSign.git
cd TruchaSign
cargo build --release
```

El binario optimizado estará disponible en `target\release\truchasign.exe`.

#### Usar el Script de Configuración

Alternativamente, usa el script de tareas PowerShell incluido para una configuración más fácil:

```powershell
# Inicializar repositorio git y crear commit inicial
.\tasks.ps1 -Task init-repo

# Configurar entorno de desarrollo local
.\tasks.ps1 -Task setup-local

# Compilar el proyecto
.\tasks.ps1 -Task build

# Validar formato y calidad del código
.\tasks.ps1 -Task validate

# Limpiar artefactos de compilación
.\tasks.ps1 -Task clean

# Publicar en GitHub
.\tasks.ps1 -Task publish
```

O simplemente ejecuta `.\tasks.ps1` para ver todas las opciones disponibles.

### 🚀 Inicio Rápido

```powershell
# Sintaxis básica
.\target\release\truchasign.exe <origen_firmado.exe> <objetivo.exe> <salida.exe>

# Ejemplo: Clonar firma de cmd.exe a payload
.\target\release\truchasign.exe C:\Windows\System32\cmd.exe .\payload.exe .\payload_signed.exe
```

#### Salida Esperada

```
[+] firma origen: offset=0x12000 size=8456 bytes
[+] inyectado va=0x4A800 size=8456 checksum=0x0004F3A1
[!] listo -> .\payload_signed.exe
```

### 🔍 Cómo Funciona

1. **Análisis DOS Header** — Lee `e_lfanew` en offset `0x3C` y valida `MZ` / `PE\0\0`
2. **Detectar Arquitectura** — Identifica formato PE32 (`0x10B`) o PE32+ (`0x20B`)
3. **Extraer Firma** — Localiza Security Data Directory (índice 4) y extrae blob WIN_CERTIFICATE
4. **Preparar Objetivo** — Rellena PE a límite de 8 bytes (requisito Authenticode)
5. **Inyectar Firma** — Añade blob de firma al PE objetivo
6. **Actualizar Headers** — Modifica `VirtualAddress` y `Size` del directorio de seguridad
7. **Recalcular Checksum** — Aplica algoritmo IMAGHELP de Microsoft (suma 16-bit con carry-fold + tamaño)

### 🏗️ Arquitectura

- **`pe_offsets`** — Resuelve offsets PE32/PE32+ con validación `MZ`/`PE`
- **`extract_signature`** — Valida y extrae blob de firma con bounds checking
- **`calculate_checksum`** — Implementa algoritmo de checksum estándar de Microsoft
- **`inject_signature`** — Maneja alineación, inyección y parcheo de headers

### ✅ Validación Post-Inyección

Verifica los resultados de la inyección:

```powershell
# Ver certificado clonado (se espera que muestre)
sigcheck.exe -i .\payload_signed.exe

# Verificar que la cadena criptográfica falla (comportamiento esperado)
Get-AuthenticodeSignature .\payload_signed.exe
truchasignl verify /pa .\payload_signed.exe
```

### ⚠️ Limitaciones

- ❌ Solo PE/PE32+ (no soporta otros formatos contenedores)
- ❌ No reconstruye certificados embebidos ni catálogos
- ❌ Validación criptográfica con `truchasignl` o `Get-AuthenticodeSignature` fallará (por diseño)
- ❌ No maneja estructuras PE corruptas

### 🤝 Cómo Contribuir

¡Las contribuciones son bienvenidas! Por favor:

1. Fork el repositorio
2. Crea una rama de característica (`git checkout -b feature/caracteristica-asombrosa`)
3. Commit tus cambios (`git commit -m 'Agregar característica asombrosa'`)
4. Push a la rama (`git push origin feature/caracteristica-asombrosa`)
5. Abre un Pull Request

### ⚖️ Aviso Legal

⚠️ **USO AUTORIZADO ÚNICAMENTE**

Esta herramienta está diseñada para:
- 🔬 Investigación de seguridad autorizada
- 🎯 Ejercicios de red team (con scope explícito)
- 🏫 Competiciones CTF (Capture The Flag)
- 🧪 Entornos de laboratorio aislados

**El uso no autorizado contra sistemas es ilegal y viola leyes de fraude informático.** El autor no se responsabiliza del mal uso o daños resultantes del acceso no autorizado.

### 📄 Licencia

Este proyecto tiene licencia **MIT OR Apache-2.0** — licencia dual para máxima compatibilidad y flexibilidad.

#### ¿Por Qué Esta Elección de Licencia?

Para una herramienta sensible de investigación de seguridad, hemos elegido un enfoque de licencia dual:

- **Licencia MIT**: Simple, permisiva y ampliamente confiada en la comunidad de código abierto
- **Apache-2.0**: Incluye garantías explícitas de patentes y protecciones legales detalladas
- **Ambas**: Permiten máxima flexibilidad manteniendo principios de código abierto

**⚠️ IMPORTANTE**: Ver el archivo [LICENSE](LICENSE) para términos adicionales sobre uso autorizado. Esta herramienta es para investigación de seguridad en contextos autorizados únicamente. El acceso no autorizado a sistemas informáticos es ilegal.

Ver el archivo [LICENSE](LICENSE) para todos los detalles.

### 📞 Soporte y Contacto

- 📧 **Correo:** [tu-email@ejemplo.com]
- 🐛 **Reportar Problemas:** [GitHub Issues](https://github.com/merovingioops/TruchaSign/issues)
- 💬 **Discusiones:** [GitHub Discussions](https://github.com/merovingioops/TruchaSign/discussions)

---

<div align="center">

**Made with ❤️ for the security community**

[![GitHub](https://img.shields.io/badge/GitHub-100000?style=for-the-badge&logo=github&logoColor=white)](https://github.com/merovingioops/TruchaSign)
[![License](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

</div>
