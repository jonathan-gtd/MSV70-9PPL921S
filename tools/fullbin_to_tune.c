/*
 * fullbin_to_tune.c
 * Extrait la region calibration 128 KB d'une ROM complete 2 MB
 * MSV70 : calibration a l'offset 0x40000 dans la ROM (adresse ECU 0x840000)
 *
 * Usage : fullbin_to_tune <input_full.bin> <output_tune.bin>
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define ROM_SIZE  (2 * 1024 * 1024)
#define CAL_OFF   0x40000
#define CAL_SIZE  (128 * 1024)

int main(int argc, char *argv[])
{
    if (argc < 3) {
        fprintf(stderr, "Usage   : fullbin_to_tune <input_full.bin> <output_tune.bin>\n");
        fprintf(stderr, "Exemple : fullbin_to_tune dump_full.bin tune.bin\n");
        return 1;
    }

    FILE *f = fopen(argv[1], "rb");
    if (!f) { perror(argv[1]); return 1; }

    fseek(f, 0, SEEK_END);
    long fsize = ftell(f);
    rewind(f);

    if (fsize < CAL_OFF + CAL_SIZE) {
        fprintf(stderr, "Erreur : fichier trop petit (%ld octets, minimum %d)\n",
                fsize, CAL_OFF + CAL_SIZE);
        fclose(f);
        return 1;
    }
    if (fsize != ROM_SIZE) {
        fprintf(stderr, "Attention : taille %ld octets (attendu %d pour 2 MB)\n",
                fsize, ROM_SIZE);
    }

    unsigned char buf[CAL_SIZE];
    fseek(f, CAL_OFF, SEEK_SET);
    size_t n = fread(buf, 1, CAL_SIZE, f);
    fclose(f);

    if (n != CAL_SIZE) {
        fprintf(stderr, "Erreur lecture : %zu / %d octets\n", n, CAL_SIZE);
        return 1;
    }

    FILE *out = fopen(argv[2], "wb");
    if (!out) { perror(argv[2]); return 1; }
    fwrite(buf, 1, CAL_SIZE, out);
    fclose(out);

    int ff = 0;
    for (int i = 0; i < CAL_SIZE; i++) if (buf[i] == 0xFF) ff++;

    printf("Input      : %s  (%.0f KB)\n", argv[1], fsize / 1024.0);
    printf("Output     : %s\n",            argv[2]);
    printf("Offset ROM : 0x%05X\n",        CAL_OFF);
    printf("Taille     : %d KB\n",         CAL_SIZE / 1024);
    printf("Programme  : %.1f KB\n",       (CAL_SIZE - ff) / 1024.0);
    printf("Efface(xFF): %.1f KB\n",       ff / 1024.0);
    return 0;
}
