%%

\{                     "{"
\}                     "}"
\(                     "("
\)                     ")"
\;                     ";"
\,                     ","

\!                     "!"
\+                     "+"
\-                     "-"
\*                     "*"
\/                     "/"
\%                     "%"
\=                     "="
\=\=                   "=="
\!\=                   "!="
\<                     "<"
\>                     ">"
\<\=                   "<="
\>\=                   ">="
\&\&                   "&&"
\|\|                   "||"

true                   "TRUEE"
false                  "FALSEE"
int                    "INT_T"
if                     "IF"
else                   "ELSE" 
while                  "WHILE"
assert                 "ASSERT"
print                  "PRINT"
halt                   "HALT" 
rand                   "RAND" 

[\pL_]\w*              "IDENT"
0|[1-9][0-9]*          "INT"

//[^\n]*               ;
[\s]+                  ;

.                      "UNMATCHED" 