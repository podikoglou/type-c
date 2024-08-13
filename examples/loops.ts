import { printf } from "stdio.h";

export function main(): number {
  let X: number = 9;

  for (let R: number = 0; R < X; R++) {
    for (let V: number = 0; V < X; V++) {
      if (R == V || R + V == X - 1 || V == X / 2 || R == X / 2) {
        printf("* ");
      } else {
        printf("  ");
      }
    }
    printf("\n");
  }

  return 0;
}
