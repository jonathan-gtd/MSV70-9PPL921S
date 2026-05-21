"""
Parse downloaded e90post HTML pages and produce a single clean HTML.
Does NOT modify the original files.
"""
import os, re, html

INPUT_DIR = os.path.join(os.path.dirname(os.path.dirname(os.path.abspath(__file__))), "pages")
OUTPUT_FILE = os.path.join(os.path.dirname(os.path.dirname(os.path.abspath(__file__))), "index.html")
TOTAL_PAGES = 114

def extract_posts(raw):
    posts = []
    blocks = re.split(r'<!-- post #(\d+) -->', raw)
    for i in range(1, len(blocks)-1, 2):
        pid = blocks[i]
        b = blocks[i+1]
        m = re.search(r'id="postcount'+pid+r'"[^>]*name="(\d+)"', b)
        num = m.group(1) if m else "?"
        m = re.search(r'</a>\s*\r?\n\s*(\d{2}-\d{2}-\d{4},\s*\d{1,2}:\d{2}\s*[AP]M)', b)
        date = m.group(1).strip() if m else ""
        m = re.search(r'<a class="bigusername"[^>]*>([^<]+)</a>', b)
        user = html.unescape(m.group(1).strip()) if m else "?"
        m = re.search(r'<div class="smallfont" style="margin-top:2px;">([^<]*)', b)
        title = html.unescape(m.group(1).strip()) if m else ""
        m = re.search(r'Drives:\s*(.+?)(?:<br|</div)', b, re.DOTALL)
        drives = html.unescape(re.sub(r'<[^>]+>','',m.group(1)).strip()) if m else ""
        m = re.search(r'Join Date:\s*([^<]+)', b)
        jd = m.group(1).strip() if m else ""
        m = re.search(r'Location:\s*([^<]+)', b)
        loc = m.group(1).strip() if m else ""
        m = re.search(r'<div id="post_message_'+pid+r'"[^>]*>(.*?)</div>\s*(?:<!--\s*/\s*message\s*-->)', b, re.DOTALL)
        body = m.group(1).strip() if m else ""
        m_attach = re.search(r'<!-- attachments -->(.*?)<!-- / attachments -->', b, re.DOTALL)
        attach_html = m_attach.group(1).strip() if m_attach else ""
        if attach_html:
            attach_html = re.sub(r'\b(?:width|height)="\d+"', '', attach_html)
            body += f'<div class="attach">{attach_html}</div>'
        body = re.sub(r'src="/', 'src="https://www.e90post.com/', body)
        body = re.sub(r'src="(?!http|/)', 'src="https://www.e90post.com/forums/', body)
        body = re.sub(r'href="/', 'href="https://www.e90post.com/', body)
        body = re.sub(r'href="(?!http|#|mailto|/)', 'href="https://www.e90post.com/forums/', body)
        posts.append(dict(id=pid,num=num,date=date,user=user,title=title,drives=drives,jd=jd,loc=loc,body=body))
    return posts

CSS = """
:root{--bg:#0f1117;--card:#1a1d27;--card2:#1e2230;--bdr:#2a2e3d;--bdr2:#3d4460;--t1:#e4e6ed;--t2:#8b90a5;--t3:#5c6078;--acc:#4f8fff;--usr:#6eb4ff;--pn:#f59e42;--qbdr:#4f8fff}
*{margin:0;padding:0;box-sizing:border-box}
body{font-family:'Inter',-apple-system,sans-serif;background:var(--bg);color:var(--t1);line-height:1.7;font-size:15px}
.ctr{max-width:960px;margin:0 auto;padding:20px 16px}
.hdr{text-align:center;padding:48px 20px 36px;border-bottom:1px solid var(--bdr);margin-bottom:12px}
.hdr h1{font-size:28px;font-weight:700;background:linear-gradient(135deg,#6eb4ff,#4f8fff,#a78bfa);-webkit-background-clip:text;-webkit-text-fill-color:transparent;background-clip:text;margin-bottom:10px}
.hdr .m{color:var(--t3);font-size:13px}
.p{background:var(--card);border:1px solid var(--bdr);border-radius:12px;margin:16px 0;overflow:hidden;transition:border-color .2s}
.p:hover{border-color:var(--bdr2)}
.p:target{border-color:var(--acc);box-shadow:0 0 0 1px rgba(79,143,255,.3)}
.ph{display:flex;align-items:center;justify-content:space-between;padding:14px 20px;background:var(--card2);border-bottom:1px solid var(--bdr);flex-wrap:wrap;gap:8px}
.phl{display:flex;align-items:center;gap:12px;flex-wrap:wrap}
.pn{font-family:'JetBrains Mono',monospace;font-size:13px;font-weight:600;color:var(--pn);background:rgba(245,158,66,.1);padding:2px 10px;border-radius:6px}
.pu{font-weight:600;font-size:15px;color:var(--usr)}
.pb2{font-size:11px;color:var(--t3);background:#252a3a;padding:2px 8px;border-radius:4px}
.pd{font-size:12px;color:#7e84a0;font-family:'JetBrains Mono',monospace}
.ui{padding:0 20px;font-size:12px;color:var(--t3);display:flex;gap:16px;flex-wrap:wrap;padding-top:10px}
.ui span+span::before{content:'·';margin-right:6px;color:var(--bdr2)}
.pb{padding:18px 22px 22px;font-size:15px;line-height:1.75;word-wrap:break-word;overflow-wrap:break-word}
.pb img{max-width:100%;height:auto;border-radius:8px;margin:8px 0;display:block}
.pb img.inlineimg{display:inline-block;margin:0 4px;vertical-align:middle;border-radius:0;width:auto}
.pb a{color:var(--acc);text-decoration:none;border-bottom:1px solid transparent;transition:border-color .15s}
.pb a:hover{border-bottom-color:var(--acc)}
.pb div[style*="margin"],.pb .bbcode_container,.pb table.tborder{background:#151823!important;border-left:3px solid var(--qbdr)!important;border-radius:6px;padding:12px 16px!important;margin:12px 0!important;color:var(--t2);font-size:14px}
.attach{margin-top:20px;padding:16px;background:var(--card2);border:1px dashed var(--bdr2);border-radius:8px;font-size:13px}
.attach fieldset{border:none}
.attach legend{font-weight:600;color:var(--usr);margin-bottom:8px}
.attach table{border-collapse:collapse}
.attach td{padding:4px;vertical-align:middle}
.attach td:first-child{white-space:nowrap}
.attach img{display:inline-block;margin:0 6px 0 0;border-radius:0;max-width:none;max-height:24px;vertical-align:middle}
.ps{display:flex;align-items:center;gap:16px;margin:32px 0;color:var(--t3);font-size:13px;font-weight:600}
.ps::before,.ps::after{content:'';flex:1;height:1px;background:linear-gradient(to right,transparent,var(--bdr2),transparent)}
.btt{position:fixed;bottom:24px;right:24px;background:var(--acc);color:#fff;border:none;border-radius:50%;width:44px;height:44px;font-size:20px;cursor:pointer;box-shadow:0 2px 12px rgba(0,0,0,.3);opacity:0;transition:opacity .3s;z-index:100}
.btt.show{opacity:1}
.ft{text-align:center;padding:32px;color:var(--t3);font-size:13px;border-top:1px solid var(--bdr);margin-top:32px}
@media(max-width:640px){.ctr{padding:8px}.pb{padding:14px 16px}.ph{padding:10px 14px}.hdr h1{font-size:22px}}
"""

def build_html(all_posts):
    h = f'''<!DOCTYPE html>
<html lang="en"><head><meta charset="UTF-8"><meta name="viewport" content="width=device-width,initial-scale=1.0">
<title>I cloned my MSV70 DME — E90Post Archive</title>
<link href="https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=JetBrains+Mono:wght@400;500&display=swap" rel="stylesheet">
<style>{CSS}</style></head><body><div class="ctr">
<div class="hdr"><h1>I cloned my MSV70 DME</h1>
<div class="m">E90Post — NA Engine / Drivetrain / Exhaust · {len(all_posts)} posts · 114 pages</div></div>\n'''

    cur_page = 0
    for p in all_posts:
        pg = ((int(p["num"])-1)//22)+1 if p["num"].isdigit() else 0
        if pg != cur_page:
            cur_page = pg
            h += f'<div class="ps">Page {cur_page}</div>\n'
        ui = ""
        uip = []
        if p["drives"]: uip.append(f'<span>{html.escape(p["drives"])}</span>')
        if p["loc"]: uip.append(f'<span>{html.escape(p["loc"])}</span>')
        if p["jd"]: uip.append(f'<span>{html.escape(p["jd"])}</span>')
        if uip: ui = f'<div class="ui">{"".join(uip)}</div>'
        badge = f'<span class="pb2">{html.escape(p["title"])}</span>' if p["title"] else ""
        h += f'''<div class="p" id="p{p["num"]}"><div class="ph"><div class="phl">
<span class="pn">#{p["num"]} (Page {pg})</span><span class="pu">{html.escape(p["user"])}</span>{badge}
</div><span class="pd">{html.escape(p["date"])}</span></div>{ui}
<div class="pb">{p["body"]}</div></div>\n'''

    h += f'''<div class="ft">Archive — {len(all_posts)} posts from e90post.com/forums/showthread.php?t=1035058</div>
</div><button class="btt" onclick="window.scrollTo({{top:0,behavior:'smooth'}})" id="btt">↑</button>
<script>window.addEventListener('scroll',()=>document.getElementById('btt').classList.toggle('show',window.scrollY>600));</script>
</body></html>'''
    return h

def main():
    all_posts = []
    for pg in range(1, TOTAL_PAGES+1):
        fp = os.path.join(INPUT_DIR, f"page_{pg:03d}.html")
        if not os.path.exists(fp):
            print(f"  [SKIP] page_{pg:03d}.html missing"); continue
        with open(fp,"r",encoding="utf-8",errors="replace") as f:
            raw = f.read()
        posts = extract_posts(raw)
        all_posts.extend(posts)
        print(f"  Page {pg:3d}/{TOTAL_PAGES}: {len(posts)} posts (total: {len(all_posts)})")

    print(f"\nWriting {len(all_posts)} posts to output...")
    out = build_html(all_posts)
    with open(OUTPUT_FILE,"w",encoding="utf-8") as f:
        f.write(out)
    mb = os.path.getsize(OUTPUT_FILE)/(1024*1024)
    print(f"Done! {OUTPUT_FILE} ({mb:.1f} MB)")

if __name__=="__main__":
    main()
