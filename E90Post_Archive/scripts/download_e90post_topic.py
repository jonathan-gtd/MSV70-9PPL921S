"""
Télécharge toutes les pages du topic e90post "I cloned my MSV70 DME"
https://www.e90post.com/forums/showthread.php?t=1035058

Sauvegarde chaque page en HTML dans le dossier e90post_t1035058/
"""

import os
import time
import urllib.request

TOPIC_ID = 1035058
TOTAL_PAGES = 114
OUTPUT_DIR = os.path.join(os.path.dirname(__file__), f"e90post_t{TOPIC_ID}")
BASE_URL = f"https://www.e90post.com/forums/showthread.php?t={TOPIC_ID}"

# User-Agent pour éviter les blocages
HEADERS = {
    "User-Agent": "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/125.0.0.0 Safari/537.36",
    "Accept": "text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8",
    "Accept-Language": "en-US,en;q=0.5",
}

# Délai entre chaque requête (secondes) pour être respectueux du serveur
DELAY = 2


def download_page(page_num: int) -> bool:
    """Télécharge une page du topic et la sauvegarde en HTML."""
    if page_num == 1:
        url = BASE_URL
    else:
        url = f"{BASE_URL}&page={page_num}"

    filename = os.path.join(OUTPUT_DIR, f"page_{page_num:03d}.html")

    # Skip si déjà téléchargé (reprise possible)
    if os.path.exists(filename) and os.path.getsize(filename) > 1000:
        print(f"  [SKIP] Page {page_num:3d}/{ TOTAL_PAGES} - déjà téléchargée")
        return True

    try:
        req = urllib.request.Request(url, headers=HEADERS)
        with urllib.request.urlopen(req, timeout=30) as response:
            html = response.read()

        with open(filename, "wb") as f:
            f.write(html)

        size_kb = len(html) / 1024
        print(f"  [OK]   Page {page_num:3d}/{TOTAL_PAGES} - {size_kb:.1f} KB")
        return True

    except Exception as e:
        print(f"  [FAIL] Page {page_num:3d}/{TOTAL_PAGES} - {e}")
        return False


def main():
    os.makedirs(OUTPUT_DIR, exist_ok=True)
    print(f"Topic: {BASE_URL}")
    print(f"Pages: {TOTAL_PAGES}")
    print(f"Dossier de sortie: {OUTPUT_DIR}")
    print("-" * 60)

    failed = []
    for page in range(1, TOTAL_PAGES + 1):
        success = download_page(page)
        if not success:
            failed.append(page)
        time.sleep(DELAY)

    print("-" * 60)
    if failed:
        print(f"Pages en échec ({len(failed)}): {failed}")
        print("Relancez le script pour réessayer (les pages déjà OK seront ignorées).")
    else:
        print(f"Terminé ! {TOTAL_PAGES} pages téléchargées dans {OUTPUT_DIR}")


if __name__ == "__main__":
    main()
