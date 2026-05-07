/*
 * msv70_tools.c  -  MSV70 bin conversion GUI
 * Compile : gcc -O2 -Wall msv70_tools.c -o msv70_tools.exe -lcomctl32 -lcomdlg32 -mwindows
 */

#define WIN32_LEAN_AND_MEAN
#include <windows.h>
#include <commctrl.h>
#include <commdlg.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdarg.h>

/* ── IDs ── */
#define ID_TAB      100
#define ID_IN_EDIT  101
#define ID_IN_BTN   102
#define ID_OUT_EDIT 103
#define ID_OUT_BTN  104
#define ID_CONVERT  105
#define ID_LOG      106
#define ID_CLEAR    107
#define TAB_N       3

/* ── Tab definitions ── */
static const struct {
    const char *title;
    const char *desc;
    const char *in_flt;
    const char *in_ext;
    const char *out_flt;
    const char *out_ext;
    const char *out_sfx;
} TABS[TAB_N] = {
    {
        "0DA -> Tune 128KB",
        "Entree  : WinKFP .0da (Intel HEX)  |  base ECU 0x840000\r\n"
        "Sortie  : calibration 128 KB",
        "WinKFP 0da (*.0da)\0*.0da\0Tous (*.*)\0*.*\0", "0da",
        "Binaire (*.bin)\0*.bin\0Tous (*.*)\0*.*\0",    "bin",
        "_tune.bin"
    },
    {
        "0PA -> Full 2MB",
        "Entree  : WinKFP .0pa (Intel HEX)  |  base ECU 0x800000\r\n"
        "Sortie  : ROM complete 2 MB",
        "WinKFP 0pa (*.0pa)\0*.0pa\0Tous (*.*)\0*.*\0", "0pa",
        "Binaire (*.bin)\0*.bin\0Tous (*.*)\0*.*\0",    "bin",
        "_full.bin"
    },
    {
        "Full 2MB -> Tune 128KB",
        "Entree  : ROM complete 2 MB\r\n"
        "Sortie  : region calibration 128 KB extraite a l'offset 0x40000",
        "Binaire (*.bin)\0*.bin\0Tous (*.*)\0*.*\0", "bin",
        "Binaire (*.bin)\0*.bin\0Tous (*.*)\0*.*\0", "bin",
        "_tune.bin"
    }
};

/* ── Globals ── */
static HWND hMain, hTab, hInEdit, hInBtn, hOutEdit, hOutBtn;
static HWND hConvert, hClear, hLog, hDesc;
static HFONT hFontUI, hFontLog;
static int curTab = 0;

/* ── Intel HEX parser ── */
static unsigned int hx2(const char *s){ unsigned int v=0; sscanf(s,"%02X",&v); return v; }
static unsigned int hx4(const char *s){ unsigned int v=0; sscanf(s,"%04X",&v); return v; }

static int parse_hex_to_buf(const char *path, unsigned char *buf,
                             unsigned long base, int size, int *gaps_out)
{
    FILE *f = fopen(path, "r");
    if (!f) return -1;
    char line[600];
    unsigned long ub = 0;
    int w = 0, gaps = 0;
    unsigned long prev_end = (unsigned long)-1;
    while (fgets(line, sizeof line, f)) {
        if (line[0] != ':') continue;
        unsigned int bc = hx2(line+1), addr = hx4(line+3), rt = hx2(line+7);
        const char *d = line+9;
        if (rt == 0x01) break;
        if (rt == 0x04) { ub = (unsigned long)hx4(d) << 16; continue; }
        if (rt != 0x00) continue;
        unsigned long abs_addr = ub + addr;
        /* gap detection within calibration range */
        long off_start = (long)abs_addr - (long)base;
        if (off_start >= 0 && off_start < size && prev_end != (unsigned long)-1
            && abs_addr > prev_end) {
            gaps++;
        }
        for (unsigned int i = 0; i < bc; i++) {
            long off = (long)(abs_addr+i) - (long)base;
            if (off >= 0 && off < size) { buf[off] = (unsigned char)hx2(d+i*2); w++; }
        }
        if (off_start < size) prev_end = abs_addr + bc;
    }
    fclose(f);
    if (gaps_out) *gaps_out = gaps;
    return w;
}

/* ── Log ── */
static void log_clear(void) { SetWindowText(hLog, ""); }

static void log_add(const char *fmt, ...)
{
    char s[512]; va_list ap; va_start(ap,fmt); vsnprintf(s,sizeof s,fmt,ap); va_end(ap);
    int n = GetWindowTextLength(hLog);
    SendMessage(hLog, EM_SETSEL, n, n);
    SendMessage(hLog, EM_REPLACESEL, 0, (LPARAM)s);
    SendMessage(hLog, EM_REPLACESEL, 0, (LPARAM)"\r\n");
}

static void log_sep(void) { log_add("-----------------------------------------"); }

/* ── Conversions ── */
static void conv_0da(const char *in, const char *out)
{
    const unsigned long BASE = 0x840000UL;
    const int           SIZE = 128*1024;
    unsigned char *buf = malloc(SIZE);
    if (!buf) { log_add("[ERR] Out of memory"); return; }
    memset(buf, 0xFF, SIZE);
    log_add("[RD]  %s", in);
    int gaps = 0;
    int w = parse_hex_to_buf(in, buf, BASE, SIZE, &gaps);
    if (w < 0) { log_add("[ERR] Cannot open input file"); free(buf); return; }
    FILE *f = fopen(out, "wb");
    if (!f) { log_add("[ERR] Cannot write: %s", out); free(buf); return; }
    fwrite(buf, 1, SIZE, f); fclose(f);
    int ff = 0; for (int i=0;i<SIZE;i++) if(buf[i]==0xFF) ff++;
    free(buf);
    log_sep();
    log_add("[OK]  %s", out);
    log_add("      Size     : 128 KB");
    log_add("      Written  : %.1f KB  (%d bytes)", (SIZE-ff)/1024.0, SIZE-ff);
    log_add("      Erased   : %.1f KB  (0xFF fill)", ff/1024.0);
    log_add("      HEX gaps : %d address gap(s) detected", gaps);
    if (gaps > 0)
        log_add("      WARNING  : gaps left as 0xFF - may differ from ECU content");
}

static void conv_0pa(const char *in, const char *out)
{
    const unsigned long BASE = 0x800000UL;
    const int           SIZE = 2*1024*1024;
    unsigned char *buf = malloc(SIZE);
    if (!buf) { log_add("[ERR] Out of memory"); return; }
    memset(buf, 0xFF, SIZE);
    log_add("[RD]  %s", in);
    int gaps = 0;
    int w = parse_hex_to_buf(in, buf, BASE, SIZE, &gaps);
    if (w < 0) { log_add("[ERR] Cannot open input file"); free(buf); return; }
    FILE *f = fopen(out, "wb");
    if (!f) { log_add("[ERR] Cannot write: %s", out); free(buf); return; }
    fwrite(buf, 1, SIZE, f); fclose(f);
    int ff = 0; for (int i=0;i<SIZE;i++) if(buf[i]==0xFF) ff++;
    free(buf);
    log_sep();
    log_add("[OK]  %s", out);
    log_add("      Size     : 2048 KB");
    log_add("      Written  : %.1f KB  (%.1f%%)", (SIZE-ff)/1024.0, (SIZE-ff)*100.0/SIZE);
    log_add("      Erased   : %.1f KB  (0xFF fill)", ff/1024.0);
    log_add("      HEX gaps : %d address gap(s) detected", gaps);
}

static void conv_full_to_tune(const char *in, const char *out)
{
    const int OFF  = 0x40000;
    const int SIZE = 128*1024;
    FILE *f = fopen(in, "rb");
    if (!f) { log_add("[ERR] Cannot open: %s", in); return; }
    fseek(f, 0, SEEK_END); long fsz = ftell(f);
    if (fsz < OFF+SIZE) {
        log_add("[ERR] File too small: %ld bytes (need %d)", fsz, OFF+SIZE);
        fclose(f); return;
    }
    if (fsz != 2*1024*1024)
        log_add("[WARN] Size %ld bytes (expected 2097152)", fsz);
    unsigned char buf[128*1024];
    fseek(f, OFF, SEEK_SET);
    size_t n = fread(buf, 1, SIZE, f); fclose(f);
    if ((int)n != SIZE) { log_add("[ERR] Read error: %zu/%d bytes", n, SIZE); return; }
    FILE *o = fopen(out, "wb");
    if (!o) { log_add("[ERR] Cannot write: %s", out); return; }
    fwrite(buf, 1, SIZE, o); fclose(o);
    int ff = 0; for (int i=0;i<SIZE;i++) if(buf[i]==0xFF) ff++;
    log_sep();
    log_add("[OK]  %s", out);
    log_add("      Source   : %.0f KB", fsz/1024.0);
    log_add("      Offset   : 0x%05X", OFF);
    log_add("      Size     : 128 KB");
    log_add("      Written  : %.1f KB", (SIZE-ff)/1024.0);
    log_add("      Erased   : %.1f KB  (0xFF fill)", ff/1024.0);
}

/* ── UI helpers ── */
static void sf_ui(HWND h)  { SendMessage(h,WM_SETFONT,(WPARAM)hFontUI, TRUE); }
static void sf_log(HWND h) { SendMessage(h,WM_SETFONT,(WPARAM)hFontLog,TRUE); }

static HWND mkedit(HWND p,int id,int x,int y,int w,int h,DWORD extra){
    HWND r=CreateWindow("EDIT","",WS_CHILD|WS_VISIBLE|WS_BORDER|ES_AUTOHSCROLL|extra,
                        x,y,w,h,p,(HMENU)(size_t)id,GetModuleHandle(0),0);
    sf_ui(r); return r;
}
static HWND mkbtn(HWND p,int id,const char*t,int x,int y,int w,int h){
    HWND r=CreateWindow("BUTTON",t,WS_CHILD|WS_VISIBLE|BS_PUSHBUTTON,
                        x,y,w,h,p,(HMENU)(size_t)id,GetModuleHandle(0),0);
    sf_ui(r); return r;
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

/* ── Resize helper: reposition controls after window resize ── */
static void layout(void)
{
    RECT cr; GetClientRect(hMain, &cr);
    int W = cr.right, H = cr.bottom;
    int M = 12, BW = 85, BH = 24;

    MoveWindow(hTab,    M, M, W-M*2, 28, TRUE);
    MoveWindow(hDesc,   M, M+34, W-M*2, 36, TRUE);

    int row1 = M+80;
    MoveWindow(hInEdit, M+58, row1,   W-M*2-58-BW-4, BH, TRUE);
    MoveWindow(hInBtn,  W-M-BW, row1, BW, BH, TRUE);

    int row2 = row1+30;
    MoveWindow(hOutEdit,M+58, row2,   W-M*2-58-BW-4, BH, TRUE);
    MoveWindow(hOutBtn, W-M-BW, row2, BW, BH, TRUE);

    int row3 = row2+34;
    MoveWindow(hConvert,M, row3,      W-M*2-BW-4, BH+2, TRUE);
    MoveWindow(hClear,  W-M-BW, row3, BW, BH+2, TRUE);

    int row4 = row3+BH+2+8;
    MoveWindow(hLog,    M, row4, W-M*2, H-row4-M, TRUE);

    /* static labels - fixed positions */
    HWND lbl;
    lbl = GetDlgItem(hMain, 200); if(lbl) MoveWindow(lbl, M, row1+3, 55, 18, TRUE);
    lbl = GetDlgItem(hMain, 201); if(lbl) MoveWindow(lbl, M, row2+3, 55, 18, TRUE);
}

/* ── WndProc ── */
static LRESULT CALLBACK WndProc(HWND hw, UINT msg, WPARAM wp, LPARAM lp)
{
    switch (msg) {

    case WM_SIZE:
        layout();
        return 0;

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

        case ID_CLEAR:
            log_clear();
            break;

        case ID_CONVERT: {
            char in[MAX_PATH], out[MAX_PATH];
            GetWindowText(hInEdit,in,MAX_PATH); GetWindowText(hOutEdit,out,MAX_PATH);
            if (!in[0])  { MessageBox(hw,"Choisissez le fichier source.","MSV70 Tools",MB_ICONWARNING); break; }
            if (!out[0]) { MessageBox(hw,"Choisissez le fichier de sortie.","MSV70 Tools",MB_ICONWARNING); break; }
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
            SetWindowText(hDesc,   TABS[curTab].desc);
            SetWindowText(hInEdit,  "");
            SetWindowText(hOutEdit, "");
            log_clear();
        }
        break; }

    case WM_CTLCOLORSTATIC:
        SetBkMode((HDC)wp, TRANSPARENT);
        return (LRESULT)GetSysColorBrush(COLOR_BTNFACE);

    case WM_GETMINMAXINFO: {
        MINMAXINFO *mm = (MINMAXINFO*)lp;
        mm->ptMinTrackSize.x = 520;
        mm->ptMinTrackSize.y = 380;
        break; }

    case WM_DESTROY: PostQuitMessage(0); break;
    default: return DefWindowProc(hw, msg, wp, lp);
    }
    return 0;
}

/* ── WinMain ── */
int WINAPI WinMain(HINSTANCE hi, HINSTANCE hp, LPSTR cmd, int show)
{
    (void)hp; (void)cmd;

    INITCOMMONCONTROLSEX icc={sizeof icc, ICC_TAB_CLASSES};
    InitCommonControlsEx(&icc);

    hFontUI  = CreateFont(15,0,0,0,FW_NORMAL,0,0,0,DEFAULT_CHARSET,
                          OUT_DEFAULT_PRECIS,CLIP_DEFAULT_PRECIS,CLEARTYPE_QUALITY,
                          DEFAULT_PITCH|FF_DONTCARE,"Segoe UI");
    hFontLog = CreateFont(14,0,0,0,FW_NORMAL,0,0,0,DEFAULT_CHARSET,
                          OUT_DEFAULT_PRECIS,CLIP_DEFAULT_PRECIS,CLEARTYPE_QUALITY,
                          FIXED_PITCH|FF_MODERN,"Consolas");

    WNDCLASS wc={0};
    wc.lpfnWndProc   = WndProc;
    wc.hInstance     = hi;
    wc.hCursor       = LoadCursor(0,IDC_ARROW);
    wc.hbrBackground = (HBRUSH)(COLOR_BTNFACE+1);
    wc.lpszClassName = "MSV70T";
    wc.hIcon         = LoadIcon(0,IDI_APPLICATION);
    RegisterClass(&wc);

    int W=700, H=480;
    RECT r={0,0,W,H};
    AdjustWindowRect(&r, WS_OVERLAPPEDWINDOW, FALSE);
    hMain = CreateWindow("MSV70T","MSV70 Tools",
                         WS_OVERLAPPEDWINDOW,
                         CW_USEDEFAULT,CW_USEDEFAULT,r.right-r.left,r.bottom-r.top,
                         0,0,hi,0);

    int M=12, BW=85, BH=24;

    /* tab */
    hTab = CreateWindow(WC_TABCONTROL,"",WS_CHILD|WS_VISIBLE|TCS_FIXEDWIDTH,
                        M,M,W-M*2,28,hMain,(HMENU)ID_TAB,hi,0);
    sf_ui(hTab);
    for(int i=0;i<TAB_N;i++){
        TCITEM ti={0}; ti.mask=TCIF_TEXT; ti.pszText=(char*)TABS[i].title;
        TabCtrl_InsertItem(hTab,i,&ti);
    }

    /* description (2-line static) */
    hDesc = CreateWindow("STATIC",TABS[0].desc,
                         WS_CHILD|WS_VISIBLE|SS_LEFT,
                         M,M+34,W-M*2,36,hMain,0,hi,0);
    sf_ui(hDesc);

    /* input row */
    int row1 = M+80;
    CreateWindow("STATIC","Source :",WS_CHILD|WS_VISIBLE|SS_LEFT,
                 M,row1+3,55,18,hMain,(HMENU)200,hi,0);
    sf_ui(GetDlgItem(hMain,200));
    hInEdit = mkedit(hMain,ID_IN_EDIT, M+58,row1, W-M*2-58-BW-4,BH, 0);
    hInBtn  = mkbtn (hMain,ID_IN_BTN, "Parcourir...", W-M-BW,row1, BW,BH);

    /* output row */
    int row2 = row1+30;
    CreateWindow("STATIC","Sortie :",WS_CHILD|WS_VISIBLE|SS_LEFT,
                 M,row2+3,55,18,hMain,(HMENU)201,hi,0);
    sf_ui(GetDlgItem(hMain,201));
    hOutEdit= mkedit(hMain,ID_OUT_EDIT,M+58,row2, W-M*2-58-BW-4,BH, 0);
    hOutBtn = mkbtn (hMain,ID_OUT_BTN,"Parcourir...",W-M-BW,row2, BW,BH);

    /* convert + clear */
    int row3 = row2+34;
    hConvert= mkbtn(hMain,ID_CONVERT,"Convertir",M,row3, W-M*2-BW-4,BH+2);
    hClear  = mkbtn(hMain,ID_CLEAR,  "Effacer",  W-M-BW,row3, BW,BH+2);

    /* log */
    int row4 = row3+BH+2+8;
    hLog = CreateWindow("EDIT","",
                        WS_CHILD|WS_VISIBLE|WS_BORDER|WS_VSCROLL|
                        ES_MULTILINE|ES_READONLY|ES_AUTOVSCROLL,
                        M,row4,W-M*2,H-row4-M,hMain,(HMENU)ID_LOG,hi,0);
    sf_log(hLog);

    ShowWindow(hMain,show); UpdateWindow(hMain);

    MSG msg;
    while(GetMessage(&msg,0,0,0)){ TranslateMessage(&msg); DispatchMessage(&msg); }
    DeleteObject(hFontUI);
    DeleteObject(hFontLog);
    return (int)msg.wParam;
}
