# type-c
A TypeScript -> C Transpiler / Compiler written in Rust.

## FAQ

### Why?

This is a stupid idea. TypeScript and C are two completely different languages with different paradigms and conventions.

That being said, I wanted to see how far I can take this stupid project. So far it seems to be going pretty well: it can compile hello world programs, use C libraries and even use pointers from within TypeScript.

### What does it look like?

A simple hello world program looks like this:

```typescript
import { printf } from "stdio.h";

export function main(argc: number, argv: string[]): number {
  printf("Hello, C!\n");
  return 0;
}
```
and compiles to this:
```c
int main(int argc, char** argv) {
printf("Hello, C\n");
return 0;
}
```

### Why is the output poorly formatted?
Because there's no need (and is too much effort) to format it from within the C codegen. You can pass it to `clang-format` like so:
```
type-c examples/hello_world.ts | clang-format
```

### Is there support for editors?
At least for now, *no!*

### How are pointers handled?
**For typing pointers**, you can use the `Pointer<T>` type and pass in the type under the pointer as the generic.

**Example:**
```typescript
let numberPtr: Pointer<Number>;
```
is compiled into:
```c
int* numberPtr;
```

**For creating pointers**, you can use the `ptr` function, and pass the object of which you want to get the address of.

**Example:**
```typescript
let a: number = 4;
let b: Pointer<number> = ptr(a);
```
is compiled into:
```c
int a = 4;
int* b = &a;
```

### Is there a standard library?
Sort of. There are some types and functions which are accessible without needing to manually import them. In the future,
we may create an npm package, similar to [`@bun/types`](https://github.com/oven-sh/bun/tree/main/packages/bun-types)
which will include all the type and function definitions such as `Pointer` and `ptr()`.
