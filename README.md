# basic-arithmetic-calculator

## BNF
expr   ::= term
         | term '+' term
         | term '-' term

term   ::= factor
         | factor '*' term
         | factor '/' term

factor ::= '(' expr ')' | digit

digit  ::= '-' [0-9]+ | [0-9]+
