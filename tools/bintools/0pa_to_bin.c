/*
 * 0pa_to_bin.c
 * Convertit un fichier WinKFP .0pa (Intel HEX) en ROM complete 2 MB
 * MSV70 : ROM a l'adresse ECU 0x800000, taille 2 MB
 *
 * Usage : 0pa_to_bin <input.0pa> <output.bin>
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define ROM_BASE 0x800000UL
#define ROM_SIZE (2 * 1024 * 1024)

static unsigned int hex2(const char *s) { unsigned int v = 0; sscanf(s, "%02X", &v); return v; }
static unsigned int hex4(const char *s) { unsigned int v = 0; sscanf(s, "%04X", &v); return v; }

int main(int argc, char *argv[])
{
    if (argc < 3) {
        fprintf(stderr, "Usage   : 0pa_to_bin <input.0pa> <output.bin>\n");
        fprintf(stderr, "Exemple : 0pa_to_bin 7577968A.0pa full_rom.bin\n");
        return 1;
    }

    unsigned char *buf = (unsigned char *)malloc(ROM_SIZE);
    if (!buf) { fprintf(stderr, "Erreur : allocation memoire\n"); return 1; }
    memset(buf, 0xFF, ROM_SIZE);

    FILE *f = fopen(argv[1], "r");
    if (!f) { perror(argv[1]); free(buf); return 1; }

    char line[600];
    unsigned long upper_base = 0;
    int written = 0;

    while (fgets(line, sizeof(line), f)) {
        if (line[0] != ':') continue;
        unsigned int byte_count = hex2(line + 1);
        unsigned int address    = hex4(line + 3);
        unsigned int rec_type   = hex2(line + 7);
        const char  *data       = line + 9;

        if (rec_type == 0x01) break;
        if (rec_type == 0x04) { upper_base = (unsigned long)hex4(data) << 16; continue; }
        if (rec_type != 0x00) continue;

        unsigned long abs_addr = upper_base + address;
        for (unsigned int i = 0; i < byte_count; i++) {
            long offset = (long)(abs_addr + i) - (long)ROM_BASE;
            if (offset >= 0 && offset < (long)ROM_SIZE) {
                buf[offset] = (unsigned char)hex2(data + i * 2);
                written++;
            }
        }
    }
    fclose(f);

    FILE *out = fopen(argv[2], "wb");
    if (!out) { perror(argv[2]); free(buf); return 1; }
    fwrite(buf, 1, ROM_SIZE, out);
    fclose(out);

    int ff = 0;
    for (int i = 0; i < (int)ROM_SIZE; i++) if (buf[i] == 0xFF) ff++;
    free(buf);

    printf("Input      : %s\n",           argv[1]);
    printf("Output     : %s\n",           argv[2]);
    printf("Taille     : %d KB\n",        ROM_SIZE / 1024);
    printf("Programme  : %.1f KB\n",      (ROM_SIZE - ff) / 1024.0);
    printf("Efface(xFF): %.1f KB\n",      ff / 1024.0);
    return 0;
}
