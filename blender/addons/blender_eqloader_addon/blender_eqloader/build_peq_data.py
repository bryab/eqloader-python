from pathlib import Path
import typing
import bpy
from mathutils import Quaternion, Matrix, Vector
from .blender_util import _convert_position, _convert_quat, _flush_collection
import eqloader


class PEQDoor(typing.TypedDict):
    doorid: int
    name: str
    opentype: int
    heading: float
    pos_x: float
    pos_y: float
    pos_z: float
    size: float
    door_param: float


def get_zone_doors(db_filepath: Path, zone_name: str):
    import sqlite3

    db = sqlite3.connect(db_filepath)
    cur = db.cursor()
    res = cur.execute(
        "SELECT * FROM doors WHERE zone LIKE ? AND min_expansion IS -1", [zone_name]
    )
    fields = [column[0] for column in res.description]
    items = res.fetchall()
    doors: list[PEQDoor] = [
        {key: value for key, value in zip(fields, item)} for item in items
    ]
    return doors


WORLD_SCALE = eqloader.WORLD_SCALE

Vector3 = typing.Tuple[float, float, float]


def _convert_peq_position(p: Vector3) -> Vector:
    gl_pos = (p[1] * -1, p[2], p[0])
    return Vector(_convert_position(gl_pos)) * WORLD_SCALE


def _convert_peq_rotation(heading: float) -> Vector:
    return Vector((0, 0, (heading / 512) * -360.0))


def find_actors() -> dict[str, bpy.types.Mesh]:
    # FIXME: Just find meshes in scene ending in _ACTORDEF
    return {}


def build_doors(doors: list[PEQDoor], actors: dict[str, bpy.types.Mesh] = None):

    if not actors:
        actors = find_actors()

    actors = {k.replace("_ACTORDEF", ""): v for k, v in actors.items()}

    col = _flush_collection("doors")

    for door in doors:
        # Doors are like actorinstances but a bit different
        # First we get the mesh
        if not door["name"] in actors:
            raise ValueError(
                f"Door {door['name']} not found in actors dict: {actors.keys()}"
            )
        mesh = actors[door["name"]]
        name = f"DOOR_{door['name']}_{door['doorid']}"
        obj = bpy.data.objects.new(name, mesh)
        obj.location = Vector(
            _convert_peq_position((door["pos_x"], door["pos_y"], door["pos_z"]))
        )
        obj.rotation_euler = _convert_peq_rotation(door["heading"])
        # obj.scale = actorinst.scale
        col.objects.link(obj)
