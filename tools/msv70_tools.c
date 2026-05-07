/*
 * msv70_tools.c  —  GUI multitool MSV70 bin conversions
 * Compile : gcc -O2 -Wall msv70_tools.c -o msv70_tools.exe -lcomctl32 -lcomdlg32 -mwindows
 *
 * Tabs :
 *   1. 0DA  -> Tune 128KB  (calibration depuis WinKFP .0da, base 0x840000)
 *   2. 0PA  -> Full  2MB   (ROM complete depuis WinKFP .0pa, base 0x800000)
 *   3. Full -> Tune 128KB  (extrait offset 0x40000 d'une ROM 2MB)
 */

#define WIN32_LEAN_AND_MEAN
#include <windows.h>
#include <commctrl.h>
#include <commdlg.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdarg.h>

/* ── IDs ─────────────────────────────────────────────────────────────────── */
#define ID_TAB      100
#define ID_IN_EDIT  101
#define ID_IN_BTN   102
#define ID_OUT_EDIT 103
#define ID_OUT_BTN  104
#define ID_CONVERT  105
#define ID_LOG      106
#define TAB_N       3

/* ── Tab definitions ─────────────────────────────────────────────────────── */
static const struct {
    const char *title;
    const char *desc;
    const char *in_flt;
    const char *in_ext;
    const char *out_flt;
    const char *out_ext;
    const char *out_sfx;    /* suffixe auto sortie */
} TABS[TAB_N] = {
    {
        "0DA → Tune 128KB",
        "Extrait la calibration 128 KB d'un fichier WinKFP .0da  (MSV70 addr ECU 0x840000)",
        "WinKFP 0da (*.0da)\0*.0da\0Tous (*.*)\0*.*\0", "0da",
        "Binaire (*.bin)\0*.bin\0Tous (*.*)\0*.*\0",    "bin",
        "_tune.bin"
    },
    {
        "0PA → Full 2MB",
        "Extrait la ROM complète 2 MB d'un fichier WinKFP .0pa  (MSV70 addr ECU 0x800000)",
        "WinKFP 0pa (*.0pa)\0*.0pa\0Tous (*.*)\0*.*\0", "0pa",
        "Binaire (*.bin)\0*.bin\0Tous (*.*)\0*.*\0",    "bin",
        "_full.bin"
    },
    {
        "Full 2MB → Tune 128KB",
        "Extrait la calibration 128 KB d'une ROM complète 2 MB  (offset 0x40000)",
        "Binaire (*.bin)\0*.bin\0Tous (*.*)\0*.*\0",    "bin",
        "Binaire (*.bin)\0*.bin\0Tous (*.*)\0*.*\0",    "bin",
        "_tune.bin"
    }
};

/* ── Globals ─────────────────────────────────────────────────────────────── */
static HWND hMain, hTab, hInEdit, hInBtn, hOutEdit, hOutBtn, hConvert, hLog, hDesc;
static HFONT hFont;
static int curTab = 0;

/* ── Intel HEX parser ────────────────────────────────────────────────────── */
static unsigned int hx2(const char *s){ unsigned int v=0; sscanf(s,"%02X",&v); return v; }
static unsigned int hx4(const char *s){ unsigned int v=0; sscanf(s,"%04X",&v); return v; }

static int parse_hex_to_buf(const char *path, unsigned char *buf,
                             unsigned long base, int size)
{
    FILE *f = fopen(path, "r");
    if (!f) return -1;
    char line[600];
    unsigned long ub = 0;
    int w = 0;
    while (fgets(line, sizeof line, f)) {
        if (line[0] != ':') continue;
        unsigned int bc = hx2(line+1), addr = hx4(line+3), rt = hx2(line+7);
        const char *d = line+9;
        if (rt == 0x01) break;
        if (rt == 0x04) { ub = (unsigned long)hx4(d) << 16; continue; }
        if (rt != 0x00) continue;
        unsigned long abs = ub + addr;
        for (unsigned int i = 0; i < bc; i++) {
            long off = (long)(abs+i) - (long)base;
            if (off >= 0 && off < size) { buf[off] = (unsigned char)hx2(d+i*2); w++; }
        }
    }
    fclose(f);
    return w;
}

/* ── Log ─────────────────────────────────────────────────────────────────── */
static void log_clear(void) { SetWindowText(hLog, ""); }

static void log_add(const char *fmt, ...)
{
    char s[512]; va_list ap; va_start(ap,fmt); vsnprintf(s,sizeof s,fmt,ap); va_end(ap);
    int n = GetWindowTextLength(hLog);
    SendMessage(hLog, EM_SETSEL, n, n);
    SendMessage(hLog, EM_REPLACESEL, 0, (LPARAM)s);
    SendMessage(hLog, EM_REPLACESEL, 0, (LPARAM)"\r\n");
}

/* ── Conversions ─────────────────────────────────────────────────────────── */
static void conv_0da(const char *in, const char *out)
{
    const unsigned long BASE = 0x840000UL;
    const int           SIZE = 128*1024;
    unsigned char *buf = malloc(SIZE);
    if (!buf) { log_add("Erreur : memoire"); return; }
    memset(buf, 0xFF, SIZE);
    log_add("Lecture %s ...", in);
    int w = parse_hex_to_buf(in, buf, BASE, SIZE);
    if (w < 0) { log_add("Erreur : impossible d'ouvrir le fichier source"); free(buf); return; }
    FILE *f = fopen(out, "wb");
    if (!f) { log_add("Erreur : impossible d'ecrire %s", out); free(buf); return; }
    fwrite(buf, 1, SIZE, f); fclose(f);
    int ff = 0; for (int i=0;i<SIZE;i++) if(buf[i]==0xFF) ff++;
    free(buf);
    log_add("OK : %s", out);
    log_add("  Taille     : 128 KB");
    log_add("  Programme  : %.1f KB  (%d octets)", (SIZE-ff)/1024.0, SIZE-ff);
    log_add("  Efface xFF : %.1f KB", ff/1024.0);
}

static void conv_0pa(const char *in, const char *out)
{
    const unsigned long BASE = 0x800000UL;
    const int           SIZE = 2*1024*1024;
    unsigned char *buf = malloc(SIZE);
    if (!buf) { log_add("Erreur : memoire"); return; }
    memset(buf, 0xFF, SIZE);
    log_add("Lecture %s ...", in);
    int w = parse_hex_to_buf(in, buf, BASE, SIZE);
    if (w < 0) { log_add("Erreur : impossible d'ouvrir le fichier source"); free(buf); return; }
    FILE *f = fopen(out, "wb");
    if (!f) { log_add("Erreur : impossible d'ecrire %s", out); free(buf); return; }
    fwrite(buf, 1, SIZE, f); fclose(f);
    int ff = 0; for (int i=0;i<SIZE;i++) if(buf[i]==0xFF) ff++;
    free(buf);
    log_add("OK : %s", out);
    log_add("  Taille     : 2048 KB");
    log_add("  Programme  : %.1f KB", (SIZE-ff)/1024.0);
    log_add("  Efface xFF : %.1f KB", ff/1024.0);
}

static void conv_full_to_tune(const char *in, const char *out)
{
    const int OFF  = 0x40000;
    const int SIZE = 128*1024;
    FILE *f = fopen(in, "rb");
    if (!f) { log_add("Erreur : impossible d'ouvrir %s", in); return; }
    fseek(f, 0, SEEK_END); long fsz = ftell(f);
    if (fsz < OFF+SIZE) {
        log_add("Erreur : fichier trop petit (%ld octets)", fsz);
        fclose(f); return;
    }
    if (fsz != 2*1024*1024)
        log_add("Attention : taille %ld (attendu 2097152)", fsz);
    unsigned char buf[128*1024];
    fseek(f, OFF, SEEK_SET);
    size_t n = fread(buf, 1, SIZE, f); fclose(f);
    if ((int)n != SIZE) { log_add("Erreur lecture : %zu/%d", n, SIZE); return; }
    FILE *o = fopen(out, "wb");
    if (!o) { log_add("Erreur : impossible d'ecrire %s", out); return; }
    fwrite(buf, 1, SIZE, o); fclose(o);
    int ff = 0; for (int i=0;i<SIZE;i++) if(buf[i]==0xFF) ff++;
    log_add("OK : %s", out);
    log_add("  Source     : %s  (%.0f KB)", in, fsz/1024.0);
    log_add("  Offset     : 0x%05X", OFF);
    log_add("  Taille     : 128 KB");
    log_add("  Programme  : %.1f KB", (SIZE-ff)/1024.0);
}

/* ── Helpers UI ──────────────────────────────────────────────────────────── */
static void sf(HWND h){ SendMessage(h,WM_SETFONT,(WPARAM)hFont,TRUE); }

static HWND mklbl(HWND p,const char*t,int x,int y,int w,int h){
    HWND r=CreateWindow("STATIC",t,WS_CHILD|WS_VISIBLE|SS_LEFT,x,y,w,h,p,0,GetModuleHandle(0),0);
    sf(r); return r;
}
static HWND mkedit(HWND p,int id,int x,int y,int w,int h){
    HWND r=CreateWindow("EDIT","",WS_CHILD|WS_VISIBLE|WS_BORDER|ES_AUTOHSCROLL,
                        x,y,w,h,p,(HMENU)(size_t)id,GetModuleHandle(0),0);
    sf(r); return r;
}
static HWND mkbtn(HWND p,int id,const char*t,int x,int y,int w,int h){
    HWND r=CreateWindow("BUTTON",t,WS_CHILD|WS_VISIBLE|BS_PUSHBUTTON,
                        x,y,w,h,p,(HMENU)(size_t)id,GetModuleHandle(0),0);
    sf(r); return r;
}

static void auto_output(void)
{
    char in[MAX_PATH]; GetWindowText(hInEdit, in, MAX_PATH);
    if (!in[0]) return;
    char out[MAX_PATH]; strncpy(out, in, MAX_PATH-20); out[MAX_PATH-20] = '\0';
    char *dot = strrchr(out, '.'); if (!dot) dot = out+strlen(out);
    strncpy(dot, TABS[curTab].out_sfx, 15);
    SetWindowText(hOutEdit, out);
}

static BOOL dlg_open(const char *flt, const char *ext, char *path)
{
    OPENFILENAME o={0}; o.lStructSize=sizeof o; o.hwndOwner=hMain;
    o.lpstrFilter=flt; o.lpstrFile=path; o.nMaxFile=MAX_PATH;
    o.lpstrDefExt=ext; o.Flags=OFN_FILEMUSTEXIST|OFN_PATHMUSTEXIST;
    return GetOpenFileName(&o);
}
static BOOL dlg_save(const char *flt, const char *ext, char *path)
{
    OPENFILENAME o={0}; o.lStructSize=sizeof o; o.hwndOwner=hMain;
    o.lpstrFilter=flt; o.lpstrFile=path; o.nMaxFile=MAX_PATH;
    o.lpstrDefExt=ext; o.Flags=OFN_OVERWRITEPROMPT;
    return GetSaveFileName(&o);
}

/* ── WndProc ─────────────────────────────────────────────────────────────── */
static LRESULT CALLBACK WndProc(HWND hw, UINT msg, WPARAM wp, LPARAM lp)
{
    switch (msg) {

    case WM_COMMAND:
        switch (LOWORD(wp)) {
        case ID_IN_BTN: {
            char p[MAX_PATH]="";
            if (dlg_open(TABS[curTab].in_flt, TABS[curTab].in_ext, p)) {
                SetWindowText(hInEdit, p);
                char cur[MAX_PATH]; GetWindowText(hOutEdit,cur,MAX_PATH);
                if (!cur[0]) auto_output();
            }
            break; }
        case ID_OUT_BTN: {
            char p[MAX_PATH]=""; GetWindowText(hOutEdit,p,MAX_PATH);
            if (dlg_save(TABS[curTab].out_flt, TABS[curTab].out_ext, p))
                SetWindowText(hOutEdit, p);
            break; }
        case ID_CONVERT: {
            char in[MAX_PATH], out[MAX_PATH];
            GetWindowText(hInEdit,in,MAX_PATH); GetWindowText(hOutEdit,out,MAX_PATH);
            if (!in[0])  { MessageBox(hw,"Choisissez le fichier source.","",MB_ICONWARNING); break; }
            if (!out[0]) { MessageBox(hw,"Choisissez le fichier destination.","",MB_ICONWARNING); break; }
            log_clear();
            log_add("=== %s ===", TABS[curTab].title);
            EnableWindow(hConvert, FALSE); SetCursor(LoadCursor(0,IDC_WAIT));
            switch(curTab) {
                case 0: conv_0da(in, out);          break;
                case 1: conv_0pa(in, out);          break;
                case 2: conv_full_to_tune(in, out); break;
            }
            SetCursor(LoadCursor(0,IDC_ARROW)); EnableWindow(hConvert, TRUE);
            break; }
        }
        break;

    case WM_NOTIFY: {
        NMHDR *nm=(NMHDR*)lp;
        if (nm->idFrom==ID_TAB && nm->code==TCN_SELCHANGE) {
            curTab = TabCtrl_GetCurSel(hTab);
            SetWindowText(hDesc,  TABS[curTab].desc);
            SetWindowText(hInEdit,  "");
            SetWindowText(hOutEdit, "");
            log_clear();
        }
        break; }

    case WM_CTLCOLORSTATIC:
        SetBkMode((HDC)wp, TRANSPARENT);
        return (LRESULT)GetSysColorBrush(COLOR_BTNFACE);

    case WM_DESTROY: PostQuitMessage(0); break;
    default: return DefWindowProc(hw, msg, wp, lp);
    }
    return 0;
}

/* ── WinMain ─────────────────────────────────────────────────────────────── */
int WINAPI WinMain(HINSTANCE hi, HINSTANCE hp, LPSTR cmd, int show)
{
    (void)hp; (void)cmd;

    INITCOMMONCONTROLSEX icc={sizeof icc, ICC_TAB_CLASSES};
    InitCommonControlsEx(&icc);

    hFont = CreateFont(15,0,0,0,FW_NORMAL,0,0,0,DEFAULT_CHARSET,
                       OUT_DEFAULT_PRECIS,CLIP_DEFAULT_PRECIS,CLEARTYPE_QUALITY,
                       DEFAULT_PITCH|FF_DONTCARE,"Segoe UI");

    WNDCLASS wc={0};
    wc.lpfnWndProc  = WndProc;
    wc.hInstance    = hi;
    wc.hCursor      = LoadCursor(0,IDC_ARROW);
    wc.hbrBackground= (HBRUSH)(COLOR_BTNFACE+1);
    wc.lpszClassName= "MSV70T";
    wc.hIcon        = LoadIcon(0,IDI_APPLICATION);
    RegisterClass(&wc);

    /* fixed-size window */
    int W=680, H=430;
    RECT r={0,0,W,H}; AdjustWindowRect(&r,WS_OVERLAPPED|WS_CAPTION|WS_SYSMENU|WS_MINIMIZEBOX,FALSE);
    hMain = CreateWindow("MSV70T","MSV70 Tools",
                         WS_OVERLAPPED|WS_CAPTION|WS_SYSMENU|WS_MINIMIZEBOX,
                         CW_USEDEFAULT,CW_USEDEFAULT,r.right-r.left,r.bottom-r.top,
                         0,0,hi,0);

    /* ── controls ── */
    int M=14, CW=W-M*2, BW=90;

    /* tab */
    hTab = CreateWindow(WC_TABCONTROL,"",WS_CHILD|WS_VISIBLE|TCS_FIXEDWIDTH,
                        M,M,CW,30,hMain,(HMENU)ID_TAB,hi,0); sf(hTab);
    for(int i=0;i<TAB_N;i++){
        TCITEM ti={0}; ti.mask=TCIF_TEXT; ti.pszText=(char*)TABS[i].title;
        TabCtrl_InsertItem(hTab,i,&ti);
    }

    /* description */
    hDesc = mklbl(hMain, TABS[0].desc, M, 52, CW, 34);

    /* input */
    mklbl(hMain,"Source :",M,100,58,20);
    hInEdit = mkedit(hMain,ID_IN_EDIT, M+62,97, CW-62-BW-6,22);
    hInBtn  = mkbtn (hMain,ID_IN_BTN, "Parcourir...", CW-BW+M+6,97, BW,22);

    /* output */
    mklbl(hMain,"Dest. :",M,130,58,20);
    hOutEdit= mkedit(hMain,ID_OUT_EDIT,M+62,127,CW-62-BW-6,22);
    hOutBtn = mkbtn (hMain,ID_OUT_BTN,"Parcourir...",CW-BW+M+6,127,BW,22);

    /* convert */
    hConvert= mkbtn(hMain,ID_CONVERT,"  Convertir  ",M,160,CW,28);

    /* log */
    hLog = CreateWindow("EDIT","",
                        WS_CHILD|WS_VISIBLE|WS_BORDER|WS_VSCROLL|
                        ES_MULTILINE|ES_READONLY|ES_AUTOVSCROLL,
                        M,200,CW,196,hMain,(HMENU)ID_LOG,hi,0); sf(hLog);

    ShowWindow(hMain,show); UpdateWindow(hMain);

    MSG msg;
    while(GetMessage(&msg,0,0,0)){ TranslateMessage(&msg); DispatchMessage(&msg); }
    DeleteObject(hFont);
    return (int)msg.wParam;
}
