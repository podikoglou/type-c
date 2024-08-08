import { printf, strlen, strcpy, strcat, malloc, free } from "stdio.h";

function reverseString(str: Pointer<string>): Pointer<string> {
  const len: number = strlen(str);
  const reversed: Pointer<string> = malloc(
    (len + 1) * sizeof(string),
  ) as Pointer<string>;

  for (let i = 0; i < len; i++) {
    reversed[i] = str[len - 1 - i];
  }
  reversed[len] = "\0";

  return reversed;
}

function concatenateStrings(
  str1: Pointer<string>,
  str2: Pointer<string>,
): Pointer<string> {
  const len1: number = strlen(str1);
  const len2: number = strlen(str2);
  const result: Pointer<string> = malloc(
    (len1 + len2 + 1) * sizeof(string),
  ) as Pointer<string>;

  strcpy(result, str1);
  strcat(result, str2);
  return result;
}

function countOccurrences(str: Pointer<string>, char: string): number {
  let count = 0;
  let ptr: Pointer<string> = str;

  while (ptr[0] !== "\0") {
    if (ptr[0] === char) {
      count++;
    }
    ptr = ptr + 1; // Pointer arithmetic
  }
  return count;
}

function modifyString(str: Pointer<string>): void {
  let ptr: Pointer<string> = str;
  while (ptr[0] !== "\0") {
    if (ptr[0] >= "a" && ptr[0] <= "z") {
      ptr[0] = String.fromCharCode(ptr[0].charCodeAt(0) - 32); // Convert to uppercase
    }
    ptr = ptr + 1; // Pointer arithmetic
  }
}

export function main(): number {
  const original: Pointer<string> = "Hello, World!";
  printf("Original: %s\n", original);

  const reversed: Pointer<string> = reverseString(original);
  printf("Reversed: %s\n", reversed);
  free(reversed);

  const str1: Pointer<string> = "Hello, ";
  const str2: Pointer<string> = "TypeScript!";
  const concatenated: Pointer<string> = concatenateStrings(str1, str2);
  printf("Concatenated: %s\n", concatenated);
  free(concatenated);

  printf(
    "Occurrences of 'l' in 'Hello, World!': %d\n",
    countOccurrences(original, "l"),
  );

  const mutableStr: Pointer<string> = malloc(
    (strlen(original) + 1) * sizeof(string),
  ) as Pointer<string>;
  strcpy(mutableStr, original);
  printf("Before modification: %s\n", mutableStr);
  modifyString(mutableStr);
  printf("After modification: %s\n", mutableStr);
  free(mutableStr);

  return 0;
}
