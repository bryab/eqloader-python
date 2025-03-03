import bpy
from pathlib import Path
import eqloader
from mathutils import Quaternion

def _convert_position(vert: tuple[float,float,float]) -> tuple[float,float,float]:
    """Convert from OpenGL coordinate to Blender coordinate"""
    return (vert[0], vert[2]*-1, vert[1])


def _convert_quat(quatvec: 'eqloader.QuatVec') -> Quaternion:
    return Quaternion([quatvec[3], quatvec[0],quatvec[1],quatvec[2]])

def _flush_collection(name: str) -> bpy.types.Collection:
    col_name = name
    if col_name in bpy.data.collections:
        bpy.data.collections.remove(bpy.data.collections[name])
    col = bpy.data.collections.new(name)
    bpy.context.scene.collection.children.link(col)
    return col

def get_blenderfile_dir() -> Path:
    current_filename = bpy.data.filepath
    assert current_filename
    return Path(current_filename).parent