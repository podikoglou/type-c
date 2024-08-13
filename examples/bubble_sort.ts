import { printf } from "stdio.h";

export function bubbleSort(arr: number[], n: number): void {
  let i: number;
  let j: number;
  let temp: number;

  for (i = 0; i < n - 1; i++) {
    for (j = 0; j < n - i - 1; j++) {
      if (arr[j] > arr[j + 1]) {
        temp = arr[j];
        arr[j] = arr[j + 1];
        arr[j + 1] = temp;
      }
    }
  }
}

export function printArray(arr: number[], n: number): void {
  let i: number;
  for (i = 0; i < n; i++) {
    printf("%d ", arr[i]);
  }
  printf("\n");
}

export function main(): number {
  const arr: number[] = [64, 34, 25, 12, 22, 11, 90];
  const n: number = 7;

  printf("Original array: ");
  printArray(arr, n);

  bubbleSort(arr, n);

  printf("Sorted array: ");
  printArray(arr, n);

  return 0;
}
