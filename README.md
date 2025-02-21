RNX2CRX
=======

[![Rust](https://github.com/rtk-rs/rnx2crx/actions/workflows/rust.yml/badge.svg)](https://github.com/rtk-rs/rnx2crx/actions/workflows/rust.yml)
[![Rust](https://github.com/rtk-rs/rnx2crx/actions/workflows/daily.yml/badge.svg)](https://github.com/rtk-rs/rnx2crx/actions/workflows/daily.yml)
[![crates.io](https://img.shields.io/crates/v/rnx2crx.svg)](https://crates.io/crates/rnx2crx)

[![License](https://img.shields.io/badge/license-MPL_2.0-orange?style=for-the-badge&logo=mozilla)](https://github.com/rtk-rs/rnx2crx/blob/main/LICENSE)

`rnx2crx` is a small command line utility to compress
your RINEX files to CRINEX (Compact RINEX) files. It is modern
replacement of the historical tool.

This tool is based on the [GeoRust/RINEX parser](https://github.com/georust/rinex).

:warning: this tool is work in progress

## Download the tool

You can download the latest version from [the release portal](https://github.com/rtk-rs/rnx2crx/releases)

## Install from Cargo

You can directly install the tool from Cargo with internet access:

```bash
cargo install rnx2crx
```

## Build from sources

Download the version you are interested in:

```bash
git clone https://github.com/rtk-rs/rnx2crx
```

And build it using cargo:

```bash
cargo build --all-features -r
```

Getting started
===============

The tool expects one input file that needs to be a valid Observation RINEX file:

```bash
rnx2crx AJAC3550.21O
Compressed AJAC3550.21D
```

By default the tool lets you know what the output file is (`stdout`).  

If that bothers you, simply use `-q` (quiet option): 

```bash
rnx2crx -q AJAC3550.21O
```

RINEX Revision
==============

The tool supports RINEX V2, V3 and V4.
It will preserve the input format by default, so standardized V3 filenames will produce a standardized file name:

```bash
rnx2crx ACOR00ESP_R_20213550000_01D_30S_MO.rnx
Compressed ACOR00ESP_R_20213550000_01D_30S_MO.crx
```

We have one option that let's you convert a V3 format to V2 directly:

```bash
rnx2crx -s ACOR00ESP_R_20213550000_01D_30S_MO.rnx
Compressed ACOR3550.21D
```

## Custom output name

You can specify a custom output location with `--prefix [directory]`: 

```bash
rnx2crx --prefix /tmp -s ACOR00ESP_R_20213550000_01D_30S_MO.rnx
Compressed /tmp/ACOR3550.21D
```

You can specify a filename yourself with `-o [filename]`,
which overrides any filename determination logic:

```bash
rnx2crx -o TEST.txt ACOR00ESP_R_20213550000_01D_30S_MO.rnx
Compressed TEST.txt
```

## Gzip files

The tool supports gzip compressed CRINEX files natively, but can only generate
"plain" CRINEX at the moment:

```bash
crx2rnx ESBC00DNK_R_20201770000_01D_30S_MO.crx.gz
Compressed ESBC00DNK_R_20201770000_01D_30S_MO.crx
```

## Compression customizations

We support several option to optimize your compression scenario:

1. `-d YYYY-MM-DD` let's you customize the CRINEX compression date.
When `-d` is not specified, we use the system time:

```bash
rnx2crx -q -d 2000-01-01 ESBC00DNK_R_20201770000_01D_30S_MO.rnx.gz
```

2. `-t HH:MM:SS` let's you customize the CRINEX compression time during that date.
When `-t` is not specified, we use the system time.

```bash
crx2rnx -q -d 2000-01-01 -t 01:02:03 ESBC00DNK_R_20201770000_01D_30S_MO.crx.gz
```

When Time is specified without Date, we use the date retrieved from system time and replace the time during that day.

## Licensing

This application is part of the [RTK-rs framework](https://github.com/rtk-rs) which
is delivered under the [Mozilla V2 Public](https://www.mozilla.org/en-US/MPL/2.0) license.
