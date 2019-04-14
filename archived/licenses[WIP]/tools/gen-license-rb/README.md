# gen-licnese-rb

[![996.icu](https://img.shields.io/badge/link-996.icu-red.svg)](https://996.icu) [![LICENSE](https://img.shields.io/badge/license-Anti%20996-blue.svg)](https://github.com/996icu/996.ICU/blob/master/LICENSE)

gen-license-rb is a 996.icu license generator implemented in Ruby, this generator is developed to generate various open-source licenses including MIT, Apache, etc. More importantly, the main purpose of this tool is to incorporate those aforesaid licenses into a brand new license: 996.icu, defined by 996.icu.

# Install
```
$ gem install gen-license
```

# Usage
```
# Common LICENSE
gen-license mit # generate mit LICENSE file in current directory

# List LICENSE codes
gen-license --list

# Conmmon LICENSE with 996ICU
gen-license mit --996icu # generate mit LICENSE with 996ICU zh-cn version
gen-license mit --996icu en-us # generate mit LICENSE with 996ICU en-us version
```

### dev
```
$ gem build gen-license.gemspec
$ gem install gen-license-0.0.1.gem
```
