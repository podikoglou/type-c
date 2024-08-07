import { printf } from "stdio.h";

function bubbleSort(arr: Pointer<number>, n: number): void {
  for (let i = 0; i < n - 1; i++) {
    for (let j = 0; j < n - i - 1; j++) {
      if (arr[j] > arr[j + 1]) {
        const temp = arr[j];
        arr[j] = arr[j + 1];
        arr[j + 1] = temp;
      }
    }
  }
}

export function main(): number {
  const arr: Pointer<number> = [64, 34, 25, 12, 22, 11, 90];
  const n: number = 7;

  printf("Original array: ");
  for (let i = 0; i < n; i++) {
    printf("%d ", arr[i]);
  }
  printf("\n");

  bubbleSort(arr, n);

  printf("Sorted array: ");
  for (let i = 0; i < n; i++) {
    printf("%d ", arr[i]);
  }
  printf("\n");

  return 0;
}
