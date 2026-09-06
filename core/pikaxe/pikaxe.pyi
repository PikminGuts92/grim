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

class Object:
    name: str
    type: str
    #props: list[tuple[str, str]]
    note: str

    @classmethod
    def get_class_name(cls) -> str: ...

class ObjectDir(Object):
    #entries: list[NamedObject] # Should this be readonly?
    #object_dir: ObjectDir
    entries: list[Object]
    sub_dirs: list[ObjectDir]

class MiloFile:
    object_dir: ObjectDir

    @staticmethod
    def load_from_file(path: str) -> MiloFile: ...

class AnimRate(IntEnum):
    k30_fps = 0
    k480_fpb = 1
    k30_fps_ui = 2
    k1_fpb = 3
    k30_fps_tutorial = 4

class Anim(Object):
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

class RndTex(Object):
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

class Rotation:
    pitch: float
    yaw: float
    roll: float

class Vector3:
    x: float
    y: float
    z: float

class Transform:
    translation: Vector3
    rotation: Rotation
    scale: Vector3

class TransConstraint(IntEnum):
    kConstraintNone = 0
    kConstraintLocalRotate = 1
    kConstraintParentWorld = 2
    kConstraintLookAtTarget = 3
    kConstraintShadowTarget = 4
    kConstraintBillboardZ = 5
    kConstraintBillboardXZ = 6
    kConstraintBillboardXYZ = 7
    kConstraintFastBillboardXYZ = 8

class RndTransformable(Object):
    local_xfm: Transform
    world_xfm: Transform
    trans_objects: list[str]
    constraint: TransConstraint
    target: str
    preserve_scale: bool
    parent: str

class NamedObject:
    name: str
    object: Object