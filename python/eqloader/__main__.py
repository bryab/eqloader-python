if __name__ == "__main__":
    import argparse
    from pathlib import Path
    from .extract import extract_all

    parser = argparse.ArgumentParser("S3D Extractor")
    parser.add_argument("archive_filepath", type=Path)
    parser.add_argument("target_dir", type=Path)
    args = parser.parse_args()
    extract_all(args.archive_filepath, args.target_dir)
