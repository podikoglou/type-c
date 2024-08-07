import { printf, scanf } from "stdio.h"

export function main(): number {
  let a: number;
  let b: number;
  let op: string;

  printf("Enter two numbers and an operation (+, -, *, /): ");
  scanf("%d %d %c", &a, &b, &op);

  switch (op) {
    case '+':
      printf("%d + %d = %d\n", a, b, a + b);
      break;
    case '-':
      printf("%d - %d = %d\n", a, b, a - b);
      break;
    case '*':
      printf("%d * %d = %d\n", a, b, a * b);
      break;
    case '/':
      if (b !== 0) {
        printf("%d / %d = %f\n", a, b, a / b);
      } else {
        printf("Error: Division by zero\n");
      }
      break;
    default:
      printf("Invalid operation\n");
  }

  return 0;
}
