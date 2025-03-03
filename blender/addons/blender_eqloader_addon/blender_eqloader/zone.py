from pathlib import Path
import logging
import typing
import bpy
import eqloader
from mathutils import Quaternion, Matrix, Vector
from .blender_util import _flush_collection, _convert_quat, _convert_position
from .image import build_images
from .material import build_materials
from .mesh import build_mesh
from .actors import build_actors

logger = logging.getLogger(__name__)


def build_zone(zone_filename: Path, actors: dict[str, bpy.types.Mesh] = None):
    logger.info(f"Attempting to build zone: {zone_filename}")
    print(dir(eqloader))

    archive = eqloader.S3DArchive(str(zone_filename))

    # Just extract all the textures

    build_images(archive)

    wld = archive.main_wld

    materials = build_materials(wld)

    _build_zone_meshes(wld)

    wld = archive.actorinst_wld
    col = _flush_collection(wld.filename)

    for actorinst in wld.actorinstances:
        # logger.info(f"Building actor instance for {actorinst.actordef_name}")
        if not actorinst.actordef_name in actors:
            raise ValueError(
                f"Actordef {actorinst.actordef_name} not in {actors.keys()}"
            )
        actor_mesh = actors[actorinst.actordef_name]
        name = f"{actorinst.actordef_name}_{actorinst.index}"
        obj = bpy.data.objects.new(name, actor_mesh)

        obj.location = Vector(
            _convert_position(actorinst.position)
        )  # + Vector(actor_mesh['center'].to_list())
        obj.rotation_mode = "QUATERNION"
        obj.rotation_quaternion = _convert_quat(actorinst.quaternion)
        obj.scale = actorinst.scale
        col.objects.link(obj)


def _build_zone_meshes(wld: eqloader.S3DWld):
    logger.info("Building zone meshes.")
    # Now build the meshes (lets ignore materials for now)

    col = _flush_collection(wld.filename)

    for eqmesh in wld.meshes:
        # logger.info(f"Building mesh: {eqmesh.name}")
        mesh = build_mesh(eqmesh, bake_position=False)
        obj = bpy.data.objects.new(eqmesh.name, mesh)
        obj.location = _convert_position(eqmesh.center)
        col.objects.link(obj)
    logger.info(f"Done building zone meshes.")
