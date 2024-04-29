timezone (tz) - English Language Timezone Conversion


# Table of Contents

1.  [Installation](#orgba9af16)
    1.  [Build from Source](#org1b2c114)
    2.  [crates.io](#org0f6e1b5)
2.  [Usage](#orgd322d81)
    1.  [As a Library](#orgc30512a)
    2.  [As a CLI](#org5f5989b)
        1.  [Arguments](#org293ad34)
        2.  [Required Arguments](#org9215a22)


<a id="orgba9af16"></a>

# Installation


<a id="org1b2c114"></a>

## Build from Source

The source code can be found at [this](https://github.com/LiquidZulu/timezone) GitHub repo. When the source has been obtained the easiest way to compile is with `cargo build --release`, then add `path/to/timezone/target/release` to your [PATH](https://en.wikipedia.org/wiki/PATH_(variable)). I do not know if this software compiles on Windows, if you have any problems with doing this [open an issue](https://github.com/LiquidZulu/timezone/issues).


<a id="org0f6e1b5"></a>

## crates.io

This software is distributed also at [crates.io](https://crates.io/crates/timezone), and should be able to be installed with `cargo install timezone`.


<a id="orgd322d81"></a>

# Usage


<a id="orgc30512a"></a>

## As a Library

It should be possible to use this software in other rust programs with `cargo add timezone`. The majority of the actual conversion logic is handled by `chrono-tz`, but `src/parse.rs` provides several methods for parsing English-language inputs, which may be useful for you.


<a id="org5f5989b"></a>

## As a CLI

    tz time origin_timezone destination_timezone day month year

For example:

    tz 1pm et bst tomorrow

will tell you what 1pm eastern US time is in British summer time tomorrow.


<a id="org293ad34"></a>

### Arguments


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


#### `origin_timezone` and `destination_timezone`

The timezones can be either a city, such as `Europe/London`, `America/Los_Angeles`, or even `US/Eastern`; or a timezone abbreviation, such as `gmt`, `est`, `aet`. A full list of available abbreviations can be found by consulting [src/convert\_timezones.rs](https://github.com/LiquidZulu/timezone/blob/main/src/convert_timezones.rs).


#### `day`, `month`, and `year`

`day`, `month`, and `year` are for the most part self-explanatory, but you can also specify `today`, `tomorrow` or `yesterday` for the `day`.


<a id="org9215a22"></a>

### Required Arguments

This software is robust, you do not have to fully-specify the conversion that you want to perform. At a minimum you can specify only the time and the origin, with the rest being assumed to be your local timezone, the current day, the current month, and the current year.

