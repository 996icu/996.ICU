from argparse import ArgumentParser, _HelpAction
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
    "unlicenses",
    "996icu-0.1",
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
        "--996icu", dest="icu", help="Expand LICENSE with 996ICU LICENSE, Choose a language vesion or default zh-cn",
        required=False, nargs="?", const="zh-cn", default=None,
        choices=["en-us", "zh-cn"]
    )

    return parser


def select_template(language_code):
    """choose a 996icu LICENSE template according to *language_code*
    """
    map_ = {
        "zh": "zh-cn",
        "zh-cn": "zh-cn",
        "zh-hans": "zh-cn",
        "en": "en-us",
        "en-us": "en-us",
    }

    template = get_data(
        __package__,
        "licenses/996.icu.template.{}.txt".format(
            map_.get(language_code, "zh-cn")
        )
    ).decode("utf-8")

    return template


def main():
    parser = getparser()
    args = parser.parse_args()

    if args.list:
        for license in LICENSES:
            print(license)

        exit(0)
    else:  # main

        # if no args input, show help and exit
        if args.code is None:
            parser.print_help()
            parser.exit()

        resource = get_data(
            __package__,
            "licenses/{code}.txt".format(code=args.code)
        ).decode("utf-8")

        if args.icu is not None:  # --996icu option enabled
            template = select_template(args.icu)

            output = template.format(
                other=args.code,
                content=resource
            ).encode("utf-8")

        else:  # common license
            output = resource.encode("utf-8")

        with open("LICENSE", "wb") as file:
            file.write(output)

        exit(0)
