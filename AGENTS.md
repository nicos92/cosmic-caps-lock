# AGENTS.md

Reglas y contexto para trabajar en este repositorio.

## Regla de idioma

- Escribir **siempre en español**: respuestas, comentarios de código y mensajes de commit.

## Qué es este proyecto

- Applet del escritorio COSMIC (Linux), basado en `pop-os/cosmic-app-template`.
- Corre dentro del panel de COSMIC (`cosmic::applet::run::<AppModel>` en `src/main.rs`), no como app independiente.
- `libcosmic` es dependencia git de pop-os/libcosmic, fijada en `Cargo.lock` (compilar requiere red o `just vendor`).

## Comandos (fuente de verdad: justfile)

- `just` — build de release (receta por defecto)
- `just run` — build + ejecución (release)
- `just check` — clippy `--all-features -W clippy::pedantic` (único gate de calidad; no hay tests)
- `just check-json` — clippy en formato JSON (para LSP)
- `just install` / `just uninstall` — instalación en el sistema (usa las variables `rootdir` y `prefix`)
- Tras reinstalar, `just install` reescribe el `.desktop` con `Exec=cosmic-caps-lock %F`. Como el `PATH` de `cosmic-session` no incluye `~/.local/bin`, mantener un symlink `~/.cargo/bin/cosmic-caps-lock` → `~/.local/bin/cosmic-caps-lock` para que el panel encuentre el binario.
- `just vendor` / `just build-vendored` — empaquetado con dependencias vendored
- `just tag <versión>` — bump de versión en Cargo.toml + commit + tag

## Formato de código

- `rustfmt.toml` define `imports_granularity = "Module"`, una opción **inestable**: con rustfmt stable se ignora en silencio y los imports quedan sin agrupar. Formatear con `cargo +nightly fmt`.
- `.zed/settings.json` ya configura rust-analyzer con clippy como check y rustfmt nightly. Si se usa otro editor, replicar eso.

## i18n (Fluent)

- Las cadenas de la UI se definen en `i18n/<código>/cosmic_caps_lock.ftl` y se usan con la macro `fl!("id")`.
- Si un `id` no existe en el `.ftl` de la fallback, el build falla.
- Para añadir un idioma: copiar `i18n/en` a `i18n/<código ISO 639-1>` y traducir; la fallback es `en` (definida en `i18n.toml`).

## Config

- `Config` (`src/config.rs`) usa `CosmicConfigEntry` con un `CONFIG_VERSION: u64` constante y `Default` manual (patrón del applet publicado `cosmic-ext-applet-sysinfo`); no usa `#[version = N]`.
- Los cambios de config los persiste la propia app con los setters del derive (`set_<campo>(handler, value)`); no se observan cambios externos (no hay `watch_config` ni feature `dbus-config` de libcosmic).
- La app recibe los flags como `config::Flags { config, config_handler }`; si falta el handler, los cambios se descartan logueando con `eprintln!`.

## LEDs (detección de Bloq Mayús/Num/Despl)

- La lectura de los indicadores usa **sysfs** (`/sys/class/leds/<dispositivo>::capslock|numlock|scrolllock/brightness`) en `src/leds.rs`, no libinput: no requiere el grupo `input` porque `brightness` es legible por cualquiera.
- La suscripción en `src/app.rs` sondea cada 200 ms con `Subscription::run(|| ...)` (este iced no tiene `run_with_id`) y solo emite `Message::LedsChanged` cuando el estado cambia.

## Branding

- `APP_ID` (`src/config.rs`) y `appid` del justfile usan el appid real **`io.github.nicos92.cosmic-caps-lock`**.
- Los archivos de `resources/` siguen la convención de applets Flatpak: `io.github.nicos92.cosmic-caps-lock.{desktop,metainfo.xml}` y `io.github.nicos92.cosmic-caps-lock-symbolic.svg`. El repo vive en `https://github.com/nnicos92/cosmic-caps-lock`.
- La config del applet se guarda en `~/.config/cosmic/io.github.nicos92.cosmic-caps-lock/` (derivada del appid).

## Distribución (Flatpak)

- El release se distribuye como **bundle `.flatpak`** en GitHub Releases (patrón de `cosmic-ext-applet-sysinfo`, self-hosted). `cargo-sources.json` + `flatpak/` son la infraestructura para build offline.
- Manifiesto raíz: `io.github.nicos92.cosmic-caps-lock.json` (runtime `org.freedesktop.Platform//25.08` + sdk rust-stable).
- `finish-args` clave del sandbox: `--filesystem=/sys/class/leds:ro` (lectura de LEDs) y `--filesystem=xdg-config/cosmic:rw` (config). **Sin** `--share=network`.
- Flujo: generar `cargo-sources.json` desde `Cargo.lock` → `flatpak-builder --user --repo=build/repo build/builddir <appid>.json` → `flatpak build-bundle build/repo cosmic-caps-lock-<ver>.flatpak <appid>`.
- **Importante:** `libcosmic` está fijada por commit en `cargo-sources.json`; si cambia `Cargo.lock`, hay que regenerar `cargo-sources.json` (`python3 flatpak/flatpak-cargo-generator.py Cargo.lock -o cargo-sources.json`, requiere aiohttp+toml o `uv`).
- `cargo-sources.json`, `build/`, `.flatpak-builder/` y `repo/` están en `.gitignore`.

## Iconos SVG (pastilla)

- La pastilla usa iconos SVG convertidos de texto a path en `resources/` (`bloqmayus.svg`, `numbloq.svg`, `bloqdezp.svg`, `default.svg`), embebidos con `include_bytes!` en `src/icons.rs`.
- El renderer de libcosmic ignora el alpha del `icon_color` (bug en `iced/wgpu/src/image/vector.rs`): atenuar el chip inactivo se hace con una mezcla sRGB **opaca** en el closure custom de `Container::custom` (mix 0.2 sobre fondo, 0.6 sobre texto/icono), no con alpha.
