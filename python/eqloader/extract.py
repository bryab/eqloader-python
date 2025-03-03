from .eqloader import S3DArchive
import logging
from pathlib import Path

logger = logging.getLogger(__name__)


def extract_all(archive_filepath: Path, target_dir: Path):
    archive = S3DArchive(str(archive_filepath))
    for filename in archive.filenames:
        output_filepath = target_dir / filename
        with open(output_filepath, "wb") as f:
            f.write(archive.get_bytes(filename))
