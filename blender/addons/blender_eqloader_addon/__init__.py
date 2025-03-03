# pyright: reportInvalidTypeForm=false
bl_info = {
    "name": "Import EverQuest S3D",
    "author": "Bryan Fordney",
    "version": (1, 0, 0),
    "blender": (4, 3, 0),
    "location": "File > Import/Export",
    "description": "Import 3d data from EverQuest .s3d files",
    "category": "Import-Export",
}

import bpy
import sys
import logging

logger = logging.getLogger(__name__)

from pathlib import Path

thisdir = str(Path(__file__).parent)
if not thisdir in sys.path:
    sys.path.append(thisdir)

if "bpy" in locals():
    import importlib

    if "blender_eqloader" in locals():
        importlib.reload(blender_eqloader)  # type: ignore

# from bpy.types import (
#     Operator,
#     Panel,
#     PropertyGroup,
# )

from bpy.props import (
    CollectionProperty,
    StringProperty,
    BoolProperty,
)

from bpy_extras.io_utils import (
    ImportHelper,
)


class ImportS3D(bpy.types.Operator, ImportHelper):
    bl_idname = "import_eq.s3d"
    bl_label = "Import EverQuest S3D"
    bl_options = {"UNDO"}

    filename_ext = ".s3d"
    filter_glob: StringProperty(
        default="*.s3d",
        options={"HIDDEN"},
        maxlen=255,
    )
    filepath: bpy.props.StringProperty(
        name="File Path",
        description="File path used for importing the S3D file",
        maxlen=1024,
    )

    import_mode: bpy.props.EnumProperty(
        items=[
            ("AUTO", "Auto", "Automatically detect"),
            ("ZONE", "Zone", "s"),
            ("ACTORS", "Actors", ""),
            ("CHAR", "Characters", ""),
            ("MESH", "Meshes", ""),
            ("MAT", "Materials+Textures", ""),
            ("TEX", "Textures", ""),
        ],
        name="Import Mode",
        description="Method of importing the file",
        default=None,
        # update=None,
        # get=None,
        # set=None
    )
    do_peq_doors: BoolProperty(
        name="Add Zone Objects from PEQ Database",
        description="If an sqlite file is found alongside the s3d, it is used to place doors in the zone",
        default=True,
    )

    def execute(self, context):
        if not bpy.data.filepath:
            raise OSError(f"The current blender file must saved")
        import os
        import eqloader
        from .blender_eqloader import (
            build_zone,
            build_actors,
            build_meshes,
            build_materials,
            build_images,
        )

        filepath = Path(self.properties.filepath)

        actors = {}

        import_mode: str = self.properties.import_mode

        if import_mode == "AUTO":
            logger.info(filepath.stem)
            if filepath.stem.endswith("_obj"):
                import_mode = "ACTORS"
            elif filepath.stem.endswith("_chr"):
                import_mode = "CHAR"
            else:
                # FIXME: Need to actually check if the selected archive is a zone archive.  If not, default to ACTORS
                import_mode = "ZONE"
        logger.info(f"Import mode: {import_mode}")

        if import_mode == "ACTORS":
            logger.info(f"Importing actors file: {filepath.name}")
            actors = build_actors(filepath)
        elif import_mode == "CHAR":
            logger.info(f"Importing character file: {filepath.name}")
            raise NotImplementedError(f"Character importers not yet implemented")
        elif import_mode == "ZONE":
            logger.info(f"Importing Zone File")
            obj_filepath = filepath.with_stem(f"{filepath.stem}_obj")
            if obj_filepath.exists():
                logger.info(f"Detected matching objects file: {obj_filepath}")
                actors = build_actors(obj_filepath)
            build_zone(filepath, actors)
            do_doors: bool = self.properties.do_peq_doors
            if actors and do_doors:
                peq_db_file = next(filepath.parent.glob("peq*.db"), None)
                if not peq_db_file:
                    logger.error(f"No PEQ database file found.")
                else:
                    from blender_eqloader.build_peq_data import (
                        get_zone_doors,
                        build_doors,
                    )

                    zone_name = filepath.stem
                    doors = get_zone_doors(peq_db_file, zone_name)
                    if not doors:
                        logger.error(f"Failed to get doors for zone {zone_name}")
                    else:
                        build_doors(doors, actors)

        else:
            archive = eqloader.S3DArchive(str(filepath))
            build_images(archive)
            if import_mode == "MESH":
                build_materials(archive.main_wld)
                build_meshes(archive.main_wld)
            elif import_mode == "MAT":
                build_materials(archive.main_wld)

        return {"FINISHED"}

    # # Draw our checkbox
    # def draw(self, context):
    #     layout = self.layout
    #     layout.use_property_split = True
    #     layout.use_property_decorate = False

    #     sfile = context.space_data
    #     layout.label(text="IMPORT PLY AS VERTS?")


def menu_func_import(self, context):
    self.layout.operator(ImportS3D.bl_idname, text="EverQuest (.s3d)")


def register():
    bpy.utils.register_class(ImportS3D)
    bpy.types.TOPBAR_MT_file_import.append(menu_func_import)


def unregister():
    bpy.utils.unregister_class(ImportS3D)
    bpy.types.TOPBAR_MT_file_import.remove(menu_func_import)


if __name__ == "__main__":
    register()
