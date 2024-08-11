import { printf } from "stdio.h";

export function print_array(arr: number[]) {
  printf("#0: %d\n", arr[0]);
  printf("#1: %d\n", arr[1]);
}

export function main(): number {
  const arr: number[] = [4, 8];

  print_array(arr);

  /* perform swap */
  const temp: number = arr[0];
  arr[0] = arr[1];
  arr[1] = temp;

  print_array(arr);

  return 0;
}
