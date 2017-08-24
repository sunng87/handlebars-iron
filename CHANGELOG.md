# Change Log

## [0.25.2] - 2017-08-24

### Changed

* Updated handlebars to 0.29.0

## [0.25.1] - 2017-07-16

### Changed

* Updated handlebars to 0.28.x

## [0.25.0] - 2017-06-09

### Changed

* Updated handlebars to 0.27.x
* Changed internal type of `DirectorySource` from `String` to `OsString`

## [0.24.1] - 2017-05-16

### Changed

* Added `Send` mark to `SourceError` inner type

## [0.24.0] - 2017-04-25

### Changed

* Updated to handlebars 0.26.x, removed rustc_serialize support.

## [0.23.1] - 2017-04-21

### Changed

* Re-export handlebars crate from handlebars-iron

## [0.23.0] - 2017-01-28

### Changed

* Updated to Serde 0.9

## [0.22.0] - 2017-01-12

### Changed

* Updated iron to 0.5

## [0.21.0] - 2016-12-31

### Changed

* Updated handlebars to 0.24
* Added `handlebars_mut()` to retrieve a writable registry reference
  from `HandlebarsEngine`, useful to register custom helpers

## [0.20.0] - 2016-12-15

### Changed

* Update handlebars to 0.23
* New partial system, use feature `partial_legacy` for previous
  partial syntax
* Update notify to 3.0, better events aggregation

## [0.19.2] - 2016-10-31

* Update handlebars to 0.22 for better error reporting

## [0.17.0] - 2016-07-27

* Update iron to 0.4

## [0.16.0] - 2016-07-27

* Update handlebars to 0.19

## [0.15.3] - 2016-06-25

### Changed

* Update handlebars to 0.18

## [0.15.2] - 2016-05-21

### Changed

* Improved performance for directory watcher. [#45]

## [0.15.1] - 2016-04-13

### Changed

* Fixed template loading on Windows. [#42]

## [0.15.0] - 2016-04-01

### Added

* Template from `catch` branch will be rendered too. [#40]

### Changed

* Handlebars data will be removed from Iron request extension map when
  we finished rendering.

## [0.14.0] - 2016-03-25

### Changed

* `HandlebarsEngine::new2` and `HandlebarsEngine::from2` are now `new`
  and `from`.
* Updated to iron 0.3.x

### Removed

* Previous `HandlebarsEngine::new` and `HandlebarsEngine::from` were
  removed.

## [0.13.1] - 2016-03-21

### Added

* `Template::with` to render some template string without having it
  from any source
