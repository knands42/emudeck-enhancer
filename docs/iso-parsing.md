# How we read a PS2 serial out of an .iso

PS2 discs use the **ISO9660** filesystem format. To get the game's serial
(e.g. `SLES-51061`), we don't scan the whole disc — we follow three fixed
pointers, each one leading to the next.

```
Sector 16 (PVD)  -->  Root Directory  -->  SYSTEM.CNF  -->  "BOOT2 = ..."
```

## Step 1 — The Primary Volume Descriptor (PVD) is always at sector 16

Every ISO9660 disc has its volume info at a fixed spot: **sector 16**
(byte offset `16 × 2048 = 0x8000`). No searching needed — this is a fixed
convention.

## Step 2 — The PVD tells us where the root directory lives

34 bytes into the PVD (absolute offset `0x8000 + 156 = 0x809C`), there's a
**directory record** describing the root folder — where it starts on disc
and how big it is.

Example, from a real Vice City ISO:

```
0000809c: 22 00 05 01 00 00 00 01 05 b0 02 00 00 00 00 02
```

| Bytes                         | Meaning            | Value |
| ----------------------------- | ------------------ | ----- |
| `22`                          | record length      | 34    |
| `05 01 00 00` (little-endian) | LBA (start sector) | 261   |
| `b0 02 00 00` (little-endian) | size in bytes      | 688   |

So the root directory's file listing is at sector 261
(`261 × 2048 = 0x82800`), 688 bytes long.

## Step 3 — Every directory is just a list of these same records, back to back

A directory record always has this shape:

```
byte 0        record length            <- how far to jump to the next record
byte 2-5      LBA (little-endian)      <- where this file/folder starts
byte 10-13    size (little-endian)     <- how big it is
byte 32       name length
byte 33..     name text
```

**The record length (byte 0) is the key.** There's no separator between
records — you read the length, use it to slice out one record, then jump
forward exactly that many bytes to land on the next one:

```
offset = start_of_directory
loop:
  record_len = data[offset]
  if record_len == 0:
      offset += 1          # padding — nudge forward and keep looking
      continue
  record = data[offset .. offset + record_len]
  # check record's name, extract LBA/size if it's the file we want
  offset += record_len      # jump straight to the next record
```

Real example — scanning the root directory found `SYSTEM.CNF;1` at
`0x82860`:

```
00082860: 3c 00 2c 43 00 00 00 00 43 2c 38 00 00 00 00 00
00082870: 00 38 68 01 09 0a 33 1d 24 00 00 00 01 00 00 01
00082880: 0c 53 59 53 54 45 4d 2e 43 4e 46 3b 31 00 00 00
```

| Bytes          | Meaning       | Value          |
| -------------- | ------------- | -------------- |
| `3c`           | record length | 60             |
| `2c 43 00 00`  | LBA           | 17196          |
| `38 00 00 00`  | size          | 56 bytes       |
| `0c`           | name length   | 12             |
| `53 59 53 ...` | name          | `SYSTEM.CNF;1` |

## Step 4 — Read SYSTEM.CNF and grab the BOOT2 line

`SYSTEM.CNF` is a tiny plain-text file. It looks like this:

```
BOOT2 = cdrom0:\SLES_510.61;1
VER = 1.00
VMODE = PAL
```

We only care about the `BOOT2` line. Extracting the serial from it is just
string manipulation:

```
"cdrom0:\SLES_510.61;1"
        -> take text after the last "\"      -> "SLES_510.61;1"
        -> drop everything from ";" onward   -> "SLES_510.61"
        -> replace "_" with "-", drop "."    -> "SLES-51061"
```

## Summary

| Step           | Fixed at                 | Tells you                   |
| -------------- | ------------------------ | --------------------------- |
| PVD            | sector 16                | where the root directory is |
| Root directory | wherever PVD points      | where `SYSTEM.CNF` is       |
| `SYSTEM.CNF`   | wherever root dir points | the `BOOT2` line            |
| `BOOT2` line   | inside that text         | the game's serial           |

Nothing here is guessed or pattern-matched — every step is a direct pointer
to the next, which is why tools like `isoinfo` or PCSX2 itself can pull
this out almost instantly.
