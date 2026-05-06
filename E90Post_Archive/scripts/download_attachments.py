import os
import re
import html
import urllib.request
import time

BASE_DIR = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
ATTACH_DIR = os.path.join(BASE_DIR, "attachments")
INDEX_PATH = os.path.join(BASE_DIR, "index.html")

os.makedirs(ATTACH_DIR, exist_ok=True)

if not os.path.exists(INDEX_PATH):
    print("Compiled index.html not found.")
    exit(1)

with open(INDEX_PATH, "r", encoding="utf-8") as f:
    html_content = f.read()

# Find all quote blocks to ignore attachments inside them
quote_pattern = r'<div class="quotePost"[^>]*>.*?</td>\s*</tr>\s*</table>\s*</div>'
quote_blocks = [(m.start(), m.end()) for m in re.finditer(quote_pattern, html_content, re.DOTALL)]

# Pattern: href="https://www.e90post.com/forums/attachment.php?...attachmentid=123..." ... >Filename</a>
pattern = r'href="(https://www.e90post.com/forums/attachment\.php\?[^"]*attachmentid=(\d+)[^"]*)"([^>]*)>([^<]+)</a>'
all_matches = list(re.finditer(pattern, html_content))

valid_matches = []
for m in all_matches:
    idx = m.start()
    in_quote = any(start <= idx <= end for start, end in quote_blocks)
    if not in_quote:
        # Save the groups needed for the loop
        valid_matches.append(m.groups())

# Deduplicate to avoid downloading the same file multiple times
unique_matches = []
seen = set()
for grp in valid_matches:
    if grp[1] not in seen:
        seen.add(grp[1])
        unique_matches.append(grp)

matches = unique_matches

HEADERS = {
    "User-Agent": "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/125.0.0.0 Safari/537.36",
    "Accept": "*/*",
}

print(f"Found {len(matches)} attachments to download.")
new_html_content = html_content

# Split into blocks to extract the post number
blocks = html_content.split('<div class="p" id="p')

for i, (raw_url, attach_id, rest_of_tag, filename) in enumerate(matches):
    url = html.unescape(raw_url)
    # Sanitize filename
    safe_filename = "".join(c for c in filename if c.isalnum() or c in " ._-").strip()
    if not safe_filename:
        safe_filename = "attachment"
    
    # We can figure out the post number by searching where this URL appears
    post_num = "?"
    for block in blocks[1:]:
        if raw_url in block:
            m_pn = re.search(r'<span class="pn">#(\d+)', block)
            if m_pn:
                post_num = m_pn.group(1)
            else:
                m_num = re.match(r'(\d+)', block)
                if m_num:
                    post_num = m_num.group(1)
            break
            
    local_filename = f"Post{post_num}_{attach_id}_{safe_filename}"
    local_path = os.path.join(ATTACH_DIR, local_filename)
    
    # Replace in HTML
    old_link = f'href="{raw_url}"{rest_of_tag}>{filename}</a>'
    new_link = f'href="attachments/{local_filename}"{rest_of_tag}>{filename}</a>'
    new_html_content = new_html_content.replace(old_link, new_link)
    
    if os.path.exists(local_path) and os.path.getsize(local_path) > 0:
        print(f"  [{i+1}/{len(matches)}] SKIP: {local_filename} already exists.")
        continue
    
    print(f"  [{i+1}/{len(matches)}] DL: {local_filename}...")
    try:
        req = urllib.request.Request(url, headers=HEADERS)
        with urllib.request.urlopen(req, timeout=30) as response:
            data = response.read()
        with open(local_path, "wb") as f:
            f.write(data)
        time.sleep(1)
    except Exception as e:
        print(f"    ERROR downloading {local_filename}: {e}")

with open(INDEX_PATH, "w", encoding="utf-8") as f:
    f.write(new_html_content)

print(f"\nDone! Attachments downloaded to {ATTACH_DIR} and HTML updated.")
