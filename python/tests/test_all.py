import pytest
from pathlib import Path
import blender_eqloader

fixtures_dir = Path(__file__).parent / 'fixtures'

def test_load_archive():
    test_filename = fixtures_dir / 'rivervale.s3d'
    archive = blender_eqloader.S3DArchive(str(test_filename))
    #print(archive.name)
    assert 'boathouse1.bmp' in archive.get_filenames()
    print(len(archive.get_filenames()))
    assert len(archive.get_filenames()) >= 87