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