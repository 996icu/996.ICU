from argparse import ArgumentParser
from pkgutil import get_data
from sys import exit

# flie basename no extension
LICENSES = [
    "agpl-3.0",
    "apache-2.0",
    "bsd-2-clause",
    "bsd-3-clause",
    "epl-2.0",
    "gpl-2.0",
    "gpl-3.0",
    "lgpl-2.1",
    "lgpl-3.0",
    "mit",
    "mpl-2.0",
    "unlicenses"
]


def getparser():
    parser = ArgumentParser(
        prog="gen-license",
        description="tools to create license file, support GitHub LICENSE code.",
    )

    parser.add_argument(
        "code", help="LICENSE Code, --list to see", choices=LICENSES,
        nargs="?", const=None
    )

    parser.add_argument(
        "--list", dest="list", help="Show supported LICENSE Codes", required=False,
        action="store_true"
    )

    parser.add_argument(
        "--996icu", dest="icu", help="Expand LICENSE with 996ICU LICENSE", required=False,
        action="store_true"
    )

    return parser


def main():
    args = getparser().parse_args()

    if args.list:
        for license in LICENSES:
            print(license)

        exit(0)
    else:  # main

        resource = get_data(
            __package__,
            "licenses/{code}.txt".format(code=args.code)
        ).decode("utf-8")

        if args.icu:  # --996icu option enabled
            template = get_data(
                __package__,
                "licenses/996.icu.template.zh-cn.txt"
            ).decode("utf-8")

            output = template.format(
                other=args.code,
                content=resource
            ).encode("utf-8")

        else:  # common license
            output = resource

        with open("LICENSE", "wb") as file:
            file.write(output)

        exit(0)
