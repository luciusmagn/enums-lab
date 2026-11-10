# Enums, patterns and error handling

Complete the `todo!()` bodies. Each `main` contains checks.

## 1. Image formats

In `src/bin/formats.rs`, implement `detect_format`. Recognize these signatures at the beginning of a byte slice:

| Format | Signature |
| --- | --- |
| PNG | `89 50 4E 47 0D 0A 1A 0A` (hexadecimal) |
| JPEG | `FF D8 FF` (hexadecimal) |
| GIF | `GIF87a` or `GIF89a` (ASCII) |

Return `Some(ImageFormat::...)` for a matching signature and `None` otherwise. Inspect only the signature.

```sh
cargo run --bin formats
```

## 2. Subtitle timestamps

In `src/bin/timestamps.rs`, implement `parse_timestamp`. Convert `HH:MM:SS,mmm` to milliseconds; `01:23:45,600` gives `5_025_600`.

Use exactly two ASCII digits for hours, minutes and seconds, and three for milliseconds. Hours range from 00 to 99; minutes and seconds from 00 to 59. Check format first, then minutes, then seconds. Return the corresponding `TimestampError` on failure.

```sh
cargo run --bin timestamps
```

## 3. Minesweeper

In `src/bin/minesweeper.rs`, implement three `Board` methods. Coordinates are zero-based, with x increasing rightward and y downward. Board construction and display are supplied.

- `neighbouring_mines` counts mines in the eight surrounding cells, excluding the selected cell. Flags do not affect the count.
- `toggle_flag` switches `Hidden` to `Flagged` and back. For `Revealed` or `Exploded`, return `AlreadyRevealed`.
- `reveal` reveals exactly one hidden cell. For a safe cell, store `Revealed(n)` and return `Safe(n)` with its neighbouring-mine count. For a mine, store `Exploded` and return `Mine`. For a flagged cell, return `Flagged`; for `Revealed` or `Exploded`, return `AlreadyRevealed`.

All three methods return `OutOfBounds` for coordinates outside the board. Errors leave the board unchanged. Display uses `#` for hidden cells, `F` for flags, `.` for revealed zeros, digits for other revealed counts, and `*` for explosions.

```sh
cargo run --bin minesweeper
```
