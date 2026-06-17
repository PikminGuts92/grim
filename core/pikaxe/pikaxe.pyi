from enum import IntEnum

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

class Bitmap:
    bpp: int
    encoding: int
    mip_maps: int
    width: int
    height: int
    bpl: int
    raw_data: list[int]

    @staticmethod
    def from_file_path(path: str) -> Bitmap: ...

    def save_to_file(self, path: str) -> None: ...

class MiloObject:
    name: str
    type: str
    #props: list[tuple[str, str]]
    note: str

    @classmethod
    def get_class_name(cls) -> str: ...

#class ObjectDir(MiloObject):
#    entries: list[MiloObject]
#    sub_dirs: list[ObjectDir]

class AnimRate(IntEnum):
    k30_fps = 0
    k480_fpb = 1
    k30_fps_ui = 2
    k1_fpb = 3
    k30_fps_tutorial = 4

class Anim(MiloObject):
    #anim_objects: list[str]
    frame: float
    rate: AnimRate

class AnimEvent[T]:
    value: T
    pos: float

class MorphPose:
    events: list[AnimEvent[float]]

class Morph(Anim):
    poses: list[MorphPose]
    normals: bool
    spline: bool
    intensity: float

class RndTex(MiloObject):
    width: int
    height: int
    bpp: int

    index_f: float
    index: int

    ext_path: str
    use_ext_path: bool

    bitmap: Bitmap | None

    def compute_image_size(self) -> int: ...

    @staticmethod
    def create_new() -> RndTex: ...

# New code...
class Object:
    name: str

class ObjectDir(Object):
    type: str