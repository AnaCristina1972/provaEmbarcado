#include <stdio.h>


int main() {
  int a =0; 
  int b =0;
  int x = 0; 
  for (int i = 0; i < 4; i++)
  {
     printf("Digite o valor 1 ou 0 para A -> ");
     scanf("%d",&a);
     printf("Digite o valor 1 ou 0 para B -> ");
     scanf("%d",&b);
     x = a & b;
     printf(" \n\n          negação \n");
     printf("%d and %d =   %d",a,b,!x);
     printf("\n\n");
  }
  
  return 0;
}