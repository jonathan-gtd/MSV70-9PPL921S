/*
 * 0da_to_bin.c
 * Extrait la calibration 128 KB d'un fichier WinKFP .0da (Intel HEX)
 * MSV70 : region calibration a l'adresse ECU 0x840000, taille 128 KB
 *
 * Usage : 0da_to_bin <input.0da> <output.bin>
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define CAL_BASE 0x840000UL
#define CAL_SIZE (128 * 1024)

static unsigned int hex2(const char *s) { unsigned int v = 0; sscanf(s, "%02X", &v); return v; }
static unsigned int hex4(const char *s) { unsigned int v = 0; sscanf(s, "%04X", &v); return v; }

int main(int argc, char *argv[])
{
    if (argc < 3) {
        fprintf(stderr, "Usage   : 0da_to_bin <input.0da> <output.bin>\n");
        fprintf(stderr, "Exemple : 0da_to_bin S7581293.0da stock_cal.bin\n");
        return 1;
    }

    unsigned char buf[CAL_SIZE];
    memset(buf, 0xFF, CAL_SIZE);

    FILE *f = fopen(argv[1], "r");
    if (!f) { perror(argv[1]); return 1; }

    char line[600];
    unsigned long upper_base = 0;
    int written = 0;

    while (fgets(line, sizeof(line), f)) {
        if (line[0] != ':') continue;
        unsigned int byte_count = hex2(line + 1);
        unsigned int address    = hex4(line + 3);
        unsigned int rec_type   = hex2(line + 7);
        const char  *data       = line + 9;

        if (rec_type == 0x01) break;                          /* EOF record */
        if (rec_type == 0x04) { upper_base = (unsigned long)hex4(data) << 16; continue; }
        if (rec_type != 0x00) continue;

        unsigned long abs_addr = upper_base + address;
        for (unsigned int i = 0; i < byte_count; i++) {
            long offset = (long)(abs_addr + i) - (long)CAL_BASE;
            if (offset >= 0 && offset < CAL_SIZE) {
                buf[offset] = (unsigned char)hex2(data + i * 2);
                written++;
            }
        }
    }
    fclose(f);

    FILE *out = fopen(argv[2], "wb");
    if (!out) { perror(argv[2]); return 1; }
    fwrite(buf, 1, CAL_SIZE, out);
    fclose(out);

    int ff = 0;
    for (int i = 0; i < CAL_SIZE; i++) if (buf[i] == 0xFF) ff++;

    printf("Input      : %s\n",           argv[1]);
    printf("Output     : %s\n",           argv[2]);
    printf("Taille     : %d KB\n",        CAL_SIZE / 1024);
    printf("Programme  : %.1f KB\n",      (CAL_SIZE - ff) / 1024.0);
    printf("Efface(xFF): %.1f KB\n",      ff / 1024.0);
    return 0;
}
