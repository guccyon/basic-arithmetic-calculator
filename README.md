# basic-arithmetic-calculator

## BNF
expr   ::= term
         | term '+' term
         | term '-' term

term   ::= factor
         | factor '*' term
         | factor '/' term

factor ::= '(' expr ')' | digit

digit  ::= '-' expr | [0-9]+
