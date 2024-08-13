import { strlen, strcpy } from "string.h";
import { printf } from "stdio.h";
import { malloc, free } from "stdlib.h";

export function main(argc: number, argv: string[]): number {
  if (argc !== 2) {
    printf("Usage: %s <string>\n", argv[0]);
    return 1;
  }

  const str: string = argv[1];
  const len: number = strlen(str);
  const reversed: string;

  for (let i: number = 0; i < len; i++) {
    reversed[i] = str[len - 1 - i];
  }

  printf("Original: %s\n", str);
  printf("Reversed: %s\n", reversed);

  return 0;
}
