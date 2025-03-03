import eqloader
import bpy
from pathlib import Path
from .mesh import build_mesh
from .image import build_images
from .material import build_materials


def _build_actor_mesh(actor: eqloader.S3DActorDef) -> bpy.types.Mesh:
    eqmesh = actor.meshes[0]
    mesh = build_mesh(eqmesh, bake_position=True)
    # # I need to store the offset so that it can be applied later to the object.
    # # This is important because this needs to happen after the object is rotated.
    # mesh['center'] = _convert_position(eqmesh.center)
    return mesh


def build_actors(filename: Path):
    archive = eqloader.S3DArchive(str(filename))

    build_images(archive)

    wld = archive.main_wld

    build_materials(wld)

    actors: dict[str, bpy.types.Mesh] = {}
    for actor in wld.actordefs:
        actor_mesh = _build_actor_mesh(actor)
        actors[actor.name] = actor_mesh

    assert actors
    return actors
