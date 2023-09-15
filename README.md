# COSMIC Design Demo

Widget toolkit showcase for testing COSMIC widgets and their styling.

## Build & Run

Required build dependencies can be referenced from the **Build-Depends** section of the [debian/rules](./debian/rules) file. Once installed, the demo can be compiled and run with `just --unstable run`. To sync with the latest changes in libcosmic, run `cargo update`. Though any breaking changes will require source code adjustments after updating the libcosmic dependency.

## License

Licensed under the [Mozilla Public License 2.0](https://choosealicense.com/licenses/mpl-2.0).

### Contribution

Any contribution intentionally submitted for inclusion in the work by you shall be licensed under the Mozilla Public License 2.0 (MPL-2.0). Each source file should have a SPDX copyright notice at the top of the file:

```
// SPDX-License-Identifier: MPL-2.0
```
