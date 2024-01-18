class ArkOffsetEntry:
    id: int
    path: str
    offset: int
    part: int
    size: int
    inflated_size: int

class Ark:
    version: int
    encryption: int | None
    entries: list[ArkOffsetEntry]

    @staticmethod
    def from_file_path(path: str) -> Ark: ...