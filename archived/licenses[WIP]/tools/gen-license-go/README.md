# gen-licnese-go

[![996.icu](https://img.shields.io/badge/link-996.icu-red.svg)](https://996.icu) [![LICENSE](https://img.shields.io/badge/license-Anti%20996-blue.svg)](https://github.com/996icu/996.ICU/blob/master/LICENSE)

gen-license-go is a 996.icu license generator implemented in Go,
this generator is developed to generate various open-source licenses including MIT, Apache, etc.
More importantly, the main purpose of this tool is to incorporate those aforesaid licenses into
a brand new license: 996.icu, defined by [996.icu](https://github.com/996icu/996.ICU).

## Usage
There are three executable files for different operating systems: macOS, Linux and Windows, located in `bin` directory, you can pick the specific bin file based on your OS, then put the `licenses` directory and `gen-license-go` file into the same path.
```sh
 # Get the help from this tool:
./gen-license-go -h

# List all supported open-source licenses:
./gen-license-go --list

# Generate a pure open-source license, take MIT for example:
./gen-license-go mit

# Get the help from command 'gen'
./gen-license-go gen -h

# Incorporate a open-source(MIT) license into the 996icu license 
# with a specific-language(en-us or zh-cn) template:
./gen-license-go gen mit --996icu en-us
```
