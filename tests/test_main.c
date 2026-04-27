#include <stdio.h>

extern int add(int a, int b);
extern int global_var;
static int local_variable = 5;

int main()
{
  int result = add(5, 3);
  printf("Result: %d\n", result);
  printf("Global Var: %d\n", global_var);
  return 0;
}
