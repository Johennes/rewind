# Contribution guide

## Environment setup

In order to build the book, you'll need [mdbook] and [mdbook-linkcheck]
installed.

```shell
cargo install mdbook mdbook-linkcheck
```

## Building the book

To build and serve the book locally, run

```shell
mdbook serve --open
```

## Manual requirements

The Rewind Repository aims to provide access to manuals of classic HiFi
devices. Contributions of material for non-audio equipment will, therefore,
not be accepted. Other projects such as [videomanuals] already cater to
different niches and we'd like to not undermine their much appreciated efforts.

Furthermore, to not stretch the greyzone of hosting copyrighted material,
contributions of manuals that are available for download or purchase from the
copyright owner will also be rejected.

If you're unsure whether or not your contribution qualifies for acceptance,
please feel welcome to open an issue to inquire.

## Thumbnails

For aesthetic reasons all manuals must be accompanied by a thumbnail of their
first page. The thumbnail must be in JPEG format, 350 pixels wide with a
1-pixel border colored in `#ccc`.

If you have [imagemagick] installed, you can generate a thumbnail with the
following command.

```shell
magick "$file.pdf[0]" -resize x348 -bordercolor "#ccc" -border 1 $file.jpg
```

## File naming convention

For the sake of consistency and convenient repository navigation, all manuals
and thumbnails must adhere to the following file name pattern:

```
$brand_$models_$type_$langs.$ext
```

- `$brand` – The device manufacturer, e.g. `sony`
- `$models` – A comma-separated list of model numbers covered in the manual,
  e.g. `cdp-411,cdp-511`
- `$type` – The document type, one of `service-manual`, `owners-manual`
- `$langs` – A comma-separated list of [ISO 639] language codes, e.g. `de,en`
- `$ext` – The file extensions, one of `pdf` (for manuals), `jpg` (for
  thumbnails)

File names must be all lowercase and must not contain any whitespace.

[ISO 639]: https://en.wikipedia.org/wiki/List_of_ISO_639_language_codes
[imagemagick]: https://imagemagick.org
[mat2]: https://0xacab.org/jvoisin/mat2
[mdbook]: https://github.com/rust-lang/mdBook
[mdbook-linkcheck]: https://github.com/Michael-F-Bryan/mdbook-linkcheck
[videomanuals]: https://github.com/Syntonie/videomanuals
