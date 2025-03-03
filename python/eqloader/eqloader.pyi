import typing
from enum import Enum

WORLD_SCALE: float
"""All Everquest position values are multiplied by this, to convert from Feet to Meters"""

QuatVec = typing.Tuple[float, float, float, float]
Vector3 = typing.Tuple[float, float, float]
Vector2 = typing.Tuple[float, float]

class S3DFace:
    @property
    def flags(self) -> int: ...
    @property
    def indices(self) -> typing.Tuple[int, int, int]: ...

class S3DMesh:
    @property
    def name(self) -> str: ...
    @property
    def flags(self) -> int: ...
    @property
    def bounds(self) -> typing.Tuple[Vector3, Vector3]: ...
    @property
    def bounds_radius(self) -> float: ...
    @property
    def center(self) -> Vector3: ...
    @property
    def vertices(self) -> typing.Sequence[Vector3]: ...
    @property
    def normals(self) -> typing.Sequence[Vector3]: ...
    @property
    def vertex_colors(self) -> typing.Sequence[Vector3]: ...
    @property
    def uvs(self) -> typing.Sequence[Vector2]: ...
    @property
    def faces(self) -> typing.Sequence[S3DFace]: ...
    @property
    def face_material_groups(self) -> typing.Sequence[typing.Tuple[int, str]]: ...

# Monkey patch
from enum import Enum

class ShaderType(Enum):
    # Used for boundaries that are not rendered. TextInfoReference can be null or have reference.
    Boundary = 0x0
    # Standard diffuse shader
    Diffuse = 0x01
    # Diffuse variant
    Diffuse2 = 0x02
    # Transparent with 0.5 blend strength
    Transparent50 = 0x05
    # Transparent with 0.25 blend strength
    Transparent25 = 0x09
    # Transparent with 0.75 blend strength
    Transparent75 = 0x0A
    # Non solid surfaces that shouldn't really be masked
    TransparentMaskedPassable = 0x07
    TransparentAdditiveUnlit = 0x0B
    TransparentMasked = 0x13
    Diffuse3 = 0x14
    Diffuse4 = 0x15
    TransparentAdditive = 0x17
    Diffuse5 = 0x19
    InvisibleUnknown = 0x53
    Diffuse6 = 0x553
    CompleteUnknown = 0x1A  # TODO: Analyze this
    Diffuse7 = 0x12
    Diffuse8 = 0x31
    InvisibleUnknown2 = 0x4B
    DiffuseSkydome = 0x0D  # Need to confirm
    TransparentSkydome = 0x0F  # Need to confirm
    TransparentAdditiveUnlitSkydome = 0x10
    InvisibleUnknown3 = 0x03
    CompleteUnknown2 = 0x06  # Found on a "floor" wall in tanarus 'thecity'

class S3DMaterial:
    @property
    def name(self) -> str: ...
    @property
    def flags(self) -> int: ...
    @property
    def texture_filenames(self) -> typing.Sequence[str]: ...
    @property
    def texture_filename(self) -> str: ...
    @property
    def shader_type_id(self) -> int: ...
    @property
    def shader_type(self) -> ShaderType: ...

class S3DActorDef:
    @property
    def name(self) -> str: ...
    @property
    def callback_name(self) -> str: ...
    @property
    def meshes(self) -> typing.Sequence[S3DMesh]: ...

class S3DActorInstance:
    @property
    def name(self) -> str: ...
    @property
    def index(self) -> int: ...
    @property
    def vertex_colors(self) -> typing.Sequence[Vector3]: ...
    @property
    def actordef_name(self) -> str: ...
    @property
    def position(self) -> Vector3: ...
    @property
    def scale(self) -> Vector3: ...
    @property
    def quaternion(self) -> QuatVec: ...
    @property
    def rotation(self) -> Vector3: ...

class S3DWld:
    @property
    def filename(self) -> str: ...
    @property
    def meshes(self) -> typing.Sequence[S3DMesh]: ...
    @property
    def materials(self) -> typing.Sequence[S3DMaterial]: ...
    @property
    def actordefs(self) -> typing.Sequence[S3DActorDef]: ...
    @property
    def actorinstances(self) -> typing.Sequence[S3DActorInstance]: ...

class S3DArchive:

    def __init__(self, archive_path: str): ...
    @property
    def filenames(self) -> typing.Sequence[str]: ...
    @property
    def main_wld(self) -> S3DWld: ...
    @property
    def actorinst_wld(self) -> S3DWld: ...
    def get_bytes(self, filename: str) -> bytes: ...
    def get_bmp_mask_color(self, filename: str) -> Vector3: ...
