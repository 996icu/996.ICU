# gen-license

Generate LICENSES. Use Python3.

All LICENSE file comes from github. [Click here](https://help.github.com/en/articles/licensing-a-repository#searching-github-by-license-type) to see the license code.

Support [996.ICU Expanded LICENSE](https://github.com/996icu/996.ICU)

# Install

```sh
    # after git clone or other download way.
    cd gen-license && python setup.py install
    # or
    cd gen-license && python setup.py install --user
```

# Usage

```sh
    # Common LICENSE
    gen-license mit # generate mit LICENSE file in current directory

    # List LICENSE codes
    gen-license --list

    # Conmmon LICENSE with 996ICU
    gen-license mit --996icu # generate mit LICENSE with 996ICU zh-cn version
    gen-license mit --996icu en-us # generate mit LICENSE with 996ICU en-us version
```
