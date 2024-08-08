// This doesn't work!
// The reason is because  some of the functions return a tuple.

import { printf } from "stdio.h";

function add(a: number, b: number): number {
  return a + b;
}

function multiply(a: number, b: number, c: number): number {
  return a * b * c;
}

function divideAndRemainder(
  dividend: number,
  divisor: number,
): [number, number] {
  return [dividend / divisor, dividend % divisor];
}

function applyOperation(
  operation: (x: number, y: number) => number,
  a: number,
  b: number,
): number {
  return operation(a, b);
}

export function main(): number {
  printf("Addition: %d\n", add(5, 3));
  printf("Multiplication: %d\n", multiply(2, 3, 4));

  const [quotient, remainder] = divideAndRemainder(17, 5);
  printf("Division: %d, Remainder: %d\n", quotient, remainder);

  printf("Apply operation (add): %d\n", applyOperation(add, 10, 20));

  return 0;
}
