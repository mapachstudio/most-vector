# Cambios respecto a la obra original

Most Vector es una obra derivada de **VectorCraft**, del equipo ArtCraft y
su comunidad (https://github.com/storytold/vectorcraft), distribuida bajo Apache-2.0.

Este archivo cumple la cláusula 4(b) de Apache-2.0, que obliga a declarar los
cambios. Los archivos que admiten comentarios lo dicen además en su cabecera.

## Qué se ha cambiado

- **La marca.** Se ha retirado la de ArtCraft —nombre, wordmark, logo y marca
  «A»— y puesto la de Grupo MOST, como exige `docs/brand/LICENSE-brand.txt`
  de la obra original para toda versión modificada.
- Nombre de producto: `VectorCraft` → `Most Vector`.
- Identificador del paquete: `ai.storyteller.vectorcraft` → `com.grupomostmexico.mostvector`.
- Extensión de archivo: `.vectorcraft` → `.mostvector`.
- Icono de la aplicación y marca de la interfaz, sustituidos por los nuestros.
- Carpeta `docs/brand/` de la obra original, eliminada.

Lo que NO se ha tocado: los nombres internos de los crates de Rust. Son
identificadores de código, no marcas presentadas al usuario.

## Archivos modificados sin aviso en cabecera

Su formato no admite comentarios:

- `crates/ui-egui/src/i18n/bg.tsv`
- `crates/ui-egui/src/i18n/bs.tsv`
- `crates/ui-egui/src/i18n/ca.tsv`
- `crates/ui-egui/src/i18n/cs.tsv`
- `crates/ui-egui/src/i18n/da.tsv`
- `crates/ui-egui/src/i18n/de.tsv`
- `crates/ui-egui/src/i18n/el.tsv`
- `crates/ui-egui/src/i18n/es.tsv`
- `crates/ui-egui/src/i18n/fi.tsv`
- `crates/ui-egui/src/i18n/fr.tsv`
- `crates/ui-egui/src/i18n/hu.tsv`
- `crates/ui-egui/src/i18n/id.tsv`
- `crates/ui-egui/src/i18n/it.tsv`
- `crates/ui-egui/src/i18n/ja.tsv`
- `crates/ui-egui/src/i18n/ka.tsv`
- `crates/ui-egui/src/i18n/ko.tsv`
- `crates/ui-egui/src/i18n/nl.tsv`
- `crates/ui-egui/src/i18n/pl.tsv`
- `crates/ui-egui/src/i18n/pt-br.tsv`
- `crates/ui-egui/src/i18n/ru.tsv`
- `crates/ui-egui/src/i18n/sv.tsv`
- `crates/ui-egui/src/i18n/tr.tsv`
- `crates/ui-egui/src/i18n/uk.tsv`
- `crates/ui-egui/src/i18n/vi.tsv`
- `crates/ui-egui/src/i18n/zh-hans.tsv`
- `crates/ui-egui/src/i18n/zh-hant.tsv`
- `crates/ui-egui/tests/fixtures/ui-before-canvas-layout.json`
- `packaging/linux/ai.storyteller.vectorcraft.metainfo.xml.in`
- `packaging/linux/ai.storyteller.vectorcraft.mime.xml`
- `packaging/macos/Info.plist.in`
- `packaging/windows/install-policy.json`
- `packaging/windows/portable.txt`

## Atribución

> Most Vector está basado en VectorCraft, del equipo ArtCraft y su
> comunidad (https://github.com/storytold/vectorcraft), bajo licencia Apache-2.0. Grupo MOST no está
> asociado con el equipo ArtCraft, que no patrocina ni avala este producto.

Total de archivos modificados: 470.
