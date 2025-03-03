from pathlib import Path
import logging
import bpy
import eqloader
from .blender_util import get_blenderfile_dir

# def _flip_indices(indices: tuple[int,int,int]):
#     return (indices[2], indices[1], indices[0])
logger = logging.getLogger(__name__)


def build_images(archive: eqloader.S3DArchive, textures_dir: Path = None):
    if not textures_dir:
        textures_dir = get_blenderfile_dir() / "tex"
    textures_dir.mkdir(exist_ok=True, parents=True)
    images: dict[str, bpy.types.Image] = {}
    for filename in archive.filenames:
        if not filename.endswith(".bmp"):
            continue
        output_filename = textures_dir / filename
        if not output_filename.exists():
            logger.info(f"Extracting image: {filename}")
            with open(output_filename, "wb") as f:
                f.write(archive.get_bytes(filename))

        image = bpy.data.images.load(
            str(output_filename), check_existing=True
        )
        # Some bitmaps use the first color of the bmp palette as the chroma key color.
        # In case we need that, we store it in the image metadata in Blender here.
        mask_color = archive.get_bmp_mask_color(filename)
        image['eq_mask_color'] = mask_color
        images[filename] = image

    return images
