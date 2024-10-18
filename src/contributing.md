# Contribution guide

## Manual requirements

The Rewind Repository aims to provide access to manuals of classic HiFi
devices. Contributions of material for non-audio equipment will, therefore,
not be accepted. Other projects such as [videomanuals] already cater to
different niches and we'd like to not undermine their much appreciated efforts.

Furthermore, to not stretch the greyzone of hosting copyrighted material, contributions of manuals that are available for download or purchase from
the copyright owner will also be rejected.

If you're unsure whether or not your contribution qualifies for acceptance,
please feel welcome to open an issue to inquire.

## File naming convention

For the sake of consistency and convenient repository navigation, all manuals
must adhere to the following file name pattern:

```
$brand_$models_$type_$langs.pdf
```

- `$brand` – The device manufacturer, e.g. `sony`
- `$models` – A comma-separated list of model numbers covered in the manual,
  e.g. `cdp-411,cdp-511`
- `$type` – The document type, one of `service-manual`, `owners-manual`
- `$langs` – A comma-separated list of [ISO 639] language codes, e.g. `de,en`

File names must be all lowercase and must not contain any whitespace.

[ISO 639]: https://en.wikipedia.org/wiki/List_of_ISO_639_language_codes
[videomanuals]: https://github.com/Syntonie/videomanuals
