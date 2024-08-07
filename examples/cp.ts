import { printf, fopen, fclose, fread, fwrite, feof } from "stdio.h";
import { exit } from "stdlib.h";

export function main(argc: number, argv: string[]): number {
  if (argc !== 3) {
    printf("Usage: %s <source> <destination>\n", argv[0]);
    return 1;
  }

  const source = fopen(argv[1], "rb");
  if (source === null) {
    printf("Error opening source file.\n");
    return 1;
  }

  const destination = fopen(argv[2], "wb");
  if (destination === null) {
    printf("Error opening destination file.\n");
    fclose(source);
    return 1;
  }

  const buffer: Pointer<number> = new Array(1024).fill(0);
  let bytesRead: number;

  while ((bytesRead = fread(buffer, 1, 1024, source)) > 0) {
    fwrite(buffer, 1, bytesRead, destination);
  }

  fclose(source);
  fclose(destination);
  printf("File copied successfully.\n");
  return 0;
}
