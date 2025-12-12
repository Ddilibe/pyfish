#!/usr/bin/env python3
    
import os
import stat

def delete_python_files_in_subdirectories(parent_directory: str) -> None:
    
    if not os.path.exists(parent_directory):
        print(f"Error: Directory '{parent_directory}' does not exist.")
        return
    if not os.path.isdir(parent_directory):
        print(f"Error: Path '{parent_directory}' is not a directory.")
        return

    print(f"Starting deletion of .py files in subdirectories of: {parent_directory}")
    files_deleted_count = 0
    directories_scanned_count = 0

    for dirpath, _, filenames in os.walk(parent_directory):
        directories_scanned_count += 1

        if dirpath == parent_directory:
            continue

        for filename in filenames:
            if filename.endswith(".py"):
                file_path: str = os.path.join(dirpath, filename)
                try:
                    if not os.access(file_path, os.W_OK):
                        os.chmod(file_path, stat.S_IWRITE)
                    os.remove(file_path)
                    print(f"Deleted: {file_path}")
                    files_deleted_count += 1
                except PermissionError:
                    print(f"Permission denied: Cannot delete '{file_path}'. Check file permissions.")
                except OSError as e:
                    print(f"Error deleting '{file_path}': {e}")
                except Exception as e:
                    print(f"An unexpected error occurred while deleting '{file_path}': {e}")

    print(f"\nDeletion process completed.")
    print(f"Scanned {directories_scanned_count} directories.")
    print(f"Successfully deleted {files_deleted_count} Python files.")


if __name__ == "__main__":
    
    test_root = "."
    delete_python_files_in_subdirectories(test_root)
