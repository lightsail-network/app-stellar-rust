from pathlib import Path

ROOT_SCREENSHOT_PATH = Path(__file__).parent.resolve()


def get_testcases_names(class_type: type) -> list[str]:
    return [
        name
        for name, function in vars(class_type).items()
        if isinstance(function, staticmethod)
    ]
