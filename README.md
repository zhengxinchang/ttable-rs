# ttable-rs

A standalone table formatter to view tabular data in console.

## Motivation

ttable is inspired by the excellent Python based tabular viewer [tabview](https://github.com/TabViewer/tabview). ttable implemented the basic function which can read stdin and format it into stdout with formatted table. This small program is useful if you just need a tabular data formatter and do not want to install other prerequisites such as Conda. 

## Installation

You just need to download the binary file to your system and make it executable by using `chmod a+x ttable`

## How to use


```bash

# Parameters
<some command> | ttable         // Will use \t as delimiter as default.
<some command> | ttable -d ','  // Will use user specified ',' as delimiter.
<some command> | ttable -r      // Will show row index.
<some command> | ttable -c      // Will show column index.
<some command> | ttable -a      // Will show row and column index.
<some command> | ttable -m 1000 // Will show the first 1000 lines. Default = 50000

# Use ttable in pipeline
<some command> | ttable [options] | <some command>

```
