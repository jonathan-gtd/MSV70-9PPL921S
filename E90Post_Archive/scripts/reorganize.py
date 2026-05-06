import os
import shutil

BASE_DIR = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
ARCHIVE_DIR = os.path.join(BASE_DIR, "E90Post_Archive")
PAGES_DIR = os.path.join(ARCHIVE_DIR, "pages")
SCRIPTS_DIR = os.path.join(ARCHIVE_DIR, "scripts")

# Create directories
os.makedirs(PAGES_DIR, exist_ok=True)
os.makedirs(SCRIPTS_DIR, exist_ok=True)

# 1. Move old pages
old_pages_dir = os.path.join(BASE_DIR, "e90post_t1035058")
if os.path.exists(old_pages_dir):
    for f in os.listdir(old_pages_dir):
        if f.endswith(".html"):
            shutil.move(os.path.join(old_pages_dir, f), os.path.join(PAGES_DIR, f))
    try:
        os.rmdir(old_pages_dir)
    except OSError:
        pass

# 2. Move scripts
for script in ["download_e90post_topic.py", "convert_e90post_topic.py"]:
    src = os.path.join(BASE_DIR, script)
    if os.path.exists(src):
        shutil.move(src, os.path.join(SCRIPTS_DIR, script))

print(f"Workspace reorganized into: {ARCHIVE_DIR}")
