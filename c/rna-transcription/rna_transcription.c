#include "rna_transcription.h"

char *to_rna(const char *dna) {
  int n = strlen(dna);
  char *rna = calloc(n, sizeof(char));

  for(int i = 0;i < n;++i) {
    if(dna[i] == 'A') 
      rna[i] = 'U';
    else if(dna[i] == 'T') 
      rna[i] = 'A';
    else if(dna[i] == 'C') 
      rna[i] = 'G';
    else if(dna[i] == 'G') 
      rna[i] = 'C';
  }

  return rna;
}
