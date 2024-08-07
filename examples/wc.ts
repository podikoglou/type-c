import { printf, fopen, fclose, fscanf } from "stdio.h"
import { exit } from "stdlib.h"

export function main(argc: number, argv: string[]): number {
  if (argc !== 2) {
    printf("Usage: %s <filename>\n", argv[0]);
    return 1;
  }

  const file = fopen(argv[1], "r");
  if (file === null) {
    printf("Error opening file.\n");
    return 1;
  }

  let wordCount: number = 0;
  let word: string;

  while (fscanf(file, "%s", &word) !== -1) {
    wordCount++;
  }

  fclose(file);
  printf("Number of words: %d\n", wordCount);
  return 0;
}
