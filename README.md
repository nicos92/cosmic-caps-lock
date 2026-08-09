# Cosmic Caps Lock

Applet para el panel de COSMIC que muestra el estado de las teclas de bloqueo del teclado: **Bloq Mayús** (Caps Lock), **Bloq Num** (Num Lock) y **Bloq Despl** (Scroll Lock).

## Instalación

### Flatpak (recomendada)

Requisitos: Flatpak con el remote de **Flathub** configurado (si no, el runtime falla con `org.freedesktop.Platform ... no se encontró`):

```sh
flatpak remote-add --if-not-exists flathub https://dl.flathub.org/repo/flathub.flatpakrepo
```

Descarga el bundle de la última versión desde [GitHub Releases](https://github.com/nicos92/cosmic-caps-lock/releases) e instálalo:

```sh
flatpak install ./cosmic-caps-lock-<versión>.flatpak
```

La primera vez pedirá descargar el runtime `org.freedesktop.Platform//25.08`. Funciona en cualquier distro con COSMIC (Pop!_OS, Fedora, Arch, NixOS…). Para que el applet aparezca en el panel, reinicia la sesión o el panel de COSMIC.

### Manual

Un [justfile](./justfile) está incluido por defecto para el ejecutor de recetas [casey/just][just].

- `just` compila la aplicación con la receta por defecto `just build-release`
- `just run` compila y ejecuta la aplicación
- `just install` instala el proyecto en el sistema
- `just vendor` crea un tarball con las dependencias
- `just build-vendored` compila con las dependencias empaquetadas desde ese tarball
- `just check` ejecuta clippy sobre el proyecto para comprobar avisos del linter
- `just check-json` puede usarlo los IDEs que soporten LSP

## Translators

[Fluent][fluent] is used for localization of the software. Fluent's translation files are found in the [i18n directory](./i18n). New translations may copy the [English (en) localization](./i18n/en) of the project, rename `en` to the desired [ISO 639-1 language code][iso-codes], and then translations can be provided for each [message identifier][fluent-guide]. If no translation is necessary, the message may be omitted.

## Packaging

Si se empaqueta para una distribución de Linux, se venden las dependencias localmente con la regla `vendor`, y se compila con las fuentes empaquetadas usando la regla `build-vendored`. Al instalar archivos, se usan las variables `rootdir` y `prefix` para cambiar las rutas de instalación.

```sh
just vendor
just build-vendored
just rootdir=debian/cosmic-caps-lock prefix=/usr install
```

Se recomienda crear un tarball de fuentes con las dependencias empaquetadas, lo que normalmente se puede hacer ejecutando `just vendor` en el sistema anfitrión antes de entrar en el entorno de build.

## Release Flatpak

Para generar el bundle `.flatpak` que se distribuye en GitHub Releases:

```sh
# Genera cargo-sources.json (fuentes offline para el build). Requiere
# python3 con aiohttp y toml, o `uv` (ver flatpak/generate-cargo-sources.sh).
python3 flatpak/flatpak-cargo-generator.py Cargo.lock -o cargo-sources.json

# Compila y empaqueta el app en un repositorio OSTree local
flatpak-builder --user --force-clean --repo=build/repo build/builddir io.github.nicos92.cosmic-caps-lock.json

# Exporta el bundle de un solo archivo para distribuir
flatpak build-bundle build/repo cosmic-caps-lock-0.1.0.flatpak io.github.nicos92.cosmic-caps-lock
```

Requisitos previos: `flatpak-builder` y los runtimes de Flathub
`org.freedesktop.{Platform,Sdk}//25.08` y `org.freedesktop.Sdk.Extension.rust-stable//25.08`.

## Developers

Developers should install [rustup][rustup] and configure their editor to use [rust-analyzer][rust-analyzer].

[fluent]: https://projectfluent.org/
[fluent-guide]: https://projectfluent.org/fluent/guide/hello.html
[iso-codes]: https://en.wikipedia.org/wiki/List_of_ISO_639-1_codes
[just]: https://github.com/casey/just
[rustup]: https://rustup.rs/
[rust-analyzer]: https://rust-analyzer.github.io/
[sccache]: https://github.com/mozilla/sccache
