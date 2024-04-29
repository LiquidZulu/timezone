
# Table of Contents

1.  [Installation](#orgb1c8dae)
    1.  [Build from Source](#orgd1c2f62)
    2.  [crates.io](#org9229c1d)
2.  [Usage](#orgfa96c82)
    1.  [As a Library](#org1eb1e0f)
    2.  [As a CLI](#orgc4e472f)
        1.  [Arguments](#orgcb4fdfc)
            1.  [`time`](#orga836325)
            2.  [`origin_timezone` and `destination_timezone`](#org9a8adc3)
            3.  [`day`, `month`, and `year`](#orgc74ca51)
        2.  [Required Arguments](#orgd4a35a7)



<a id="orgb1c8dae"></a>

# Installation


<a id="orgd1c2f62"></a>

## Build from Source

The source code can be found at [this](https://github.com/LiquidZulu/timezone) GitHub repo. When the source has been obtained the easiest way to compile is with `cargo build --release`, then add `path/to/timezone/target/release` to your [PATH](https://en.wikipedia.org/wiki/PATH_(variable)). I do not know if this software compiles on Windows, if you have any problems with doing this [open an issue](https://github.com/LiquidZulu/timezone/issues).


<a id="org9229c1d"></a>

## crates.io

This software is distributed also at [crates.io](https://crates.io/crates/timezone), and should be able to be installed with `cargo install timezone`.


<a id="orgfa96c82"></a>

# Usage


<a id="org1eb1e0f"></a>

## As a Library

It should be possible to use this software in other rust programs with `cargo add timezone`. The majority of the actual conversion logic is handled by `chrono-tz`, but `src/parse.rs` provides several methods for parsing English-language inputs, which may be useful for you.


<a id="orgc4e472f"></a>

## As a CLI

    tz time origin_timezone destination_timezone day month year

For example:

    tz 1pm et bst tomorrow

will tell you what 1pm eastern US time is in British summer time tomorrow.


<a id="orgcb4fdfc"></a>

### Arguments


<a id="orga836325"></a>

#### `time`

`time` should be in one of the following formats:

<table border="2" cellspacing="0" cellpadding="6" rules="groups" frame="hsides">


<colgroup>
<col  class="org-left" />

<col  class="org-left" />
</colgroup>
<thead>
<tr>
<th scope="col" class="org-left">Format</th>
<th scope="col" class="org-left">Examples</th>
</tr>
</thead>

<tbody>
<tr>
<td class="org-left">SimpleAmPm</td>
<td class="org-left">1am, 10pm</td>
</tr>


<tr>
<td class="org-left">FullAmPm</td>
<td class="org-left">12:24am, 6:30pm</td>
</tr>


<tr>
<td class="org-left">MilitaryColon</td>
<td class="org-left">07:00, 13:52</td>
</tr>


<tr>
<td class="org-left">Military</td>
<td class="org-left">0900, 1634</td>
</tr>
</tbody>
</table>


<a id="org9a8adc3"></a>

#### `origin_timezone` and `destination_timezone`

The timezones can be either a city, such as `Europe/London`, `America/Los_Angeles`, or even `US/Eastern`; or a timezone abbreviation, such as `gmt`, `est`, `aet`. A full list of available abbreviations can be found by consulting [src/convert<sub>timezones.rs</sub>](https://github.com/LiquidZulu/timezone/blob/main/src/convert_timezones.rs).


<a id="orgc74ca51"></a>

#### `day`, `month`, and `year`

`day`, `month`, and `year` are for the most part self-explanatory, but you can also specify `today`, `tomorrow` or `yesterday` for the `day`.


<a id="orgd4a35a7"></a>

### Required Arguments

This software is robust, you do not have to fully-specify the conversion that you want to perform. At a minimum you can specify only the time and the origin, with the rest being assumed to be your local timezone, the current day, the current month, and the current year.

