# provaEmbarcado
Prova do dia 08 de out 2021
## Questão 1
![imagem daPrimeira questão](https://github.com/AnaCristina1972/provaEmbarcado/blob/main/imagemQuesta1.png)
##### resposta Circuito:
![imagem da resporta](https://github.com/AnaCristina1972/provaEmbarcado/blob/main/imagemDoCircuito_Questao1.png)
##### resposta Codigo VHDL:
~~~vhdl
library IEEE;
use IEEE.std_logic_1164.all;


entity porta_nand is
    Port ( 
           A : in  std_logic;      
           B : in  std_logic;      
           X : out  std_logic;  );  
         
end porta_nand;






library IEEE;
use IEEE.std_logic_1164.all;

architecture Prova of porta_nand is
begin
X <= (A nand B);    

end Prova;
~~~

## Questão 2
##### resposta Codigo Rust:
~~~rust
<<<<<<< HEAD
fn main(){

   
    let mut x:i32 =0;
    let mut a:i32 =1;
    let mut b:i32 =1;
    
    x = a & b;
    println!("{:0b} and {:0b} = {:0b}", a, b,!x);
    
    a = 1;
    b = 0;
    x = a & b;
    println!("{:0b} and {:0b} = {:0b}", a, b,!x);
    
    a=0;
    b=1;
    x = a & b;
    println!("{:0b} and {:0b} = {:0b}", a, b,!x);
    
     a=0;
     b=0;
    x = a & b;
    println!("{:0b} and {:0b} = {:0b}", a, b,!x);
=======
ffn  main () {

   
    deixe  mut x: i32  = 0 ;
    deixe  mut a: i32  = 1 ;
    deixe  mut b: i32  = 1 ;
    
    x = a & b;
    println! ( "{: 0b} e {: 0b} = {: 0b}" , a, b ,! X);
    
    a =  1 ;
    b =  0 ;
    x = a & b;
    println! ( "{: 0b} e {: 0b} = {: 0b}" , a, b ,! X);
    
    a = 0 ;
    b = 1 ;
    x = a & b;
    println! ( "{: 0b} e {: 0b} = {: 0b}" , a, b ,! X);
    
     a = 0 ;
     b = 0 ;
    x = a & b;
    println! ( "{: 0b} e {: 0b} = {: 0b}" , a, b ,! X);
>>>>>>> fe41aaa4537fb50981ec6aedd3bcc4a11e51832e


}
~~~

## Questão 3
##### resposta:
![ imagem da questão 3](https://github.com/AnaCristina1972/provaEmbarcado/blob/main/imagemQuestao3.png)
## Questão 4
![imagem da questão 4](https://github.com/AnaCristina1972/provaEmbarcado/blob/main/imagem4.png)
##### resposta:
Na figura acima mostra uma implementação de hardware / software, onde é implementado um algoritmo e conduzido em um microprocessador através da camada de software. Esta camada gerencia todas as operações necessárias para execução do programa. Caso necessário, alguma operação na camada de hardware o software direcionado para uma camada de disco rígido que pode realizar tal operação, para que a execução tenha maior desempenho. A camada de software fica fazendo esse gerenciamento entre hardware e software.

## Questão 5
##### resposta:
~~~c
# inclui  < stdio.h >


int  main () {
  int a = 0 ;
  int b = 0 ;
  int x = 0 ;
  para ( int i = 0 ; i < 4 ; i ++)
  {
     printf ( " Digite o valor 1 ou 0 para A -> " );
     scanf ( " % d " , & a);
     printf ( " Digite o valor 1 ou 0 para B -> " );
     scanf ( " % d " , & b);
     x = a & b;
     printf ( "  \ n \ n           negação \ n " );
     printf ( " % d e % d =    % d " , a, b,! x);
     printf ( " \ n \ n " );
  }
  
  return  0 ;
}
~~~









