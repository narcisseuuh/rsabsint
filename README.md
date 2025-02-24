# RSAbsInt : Analyseur statique par Interprétation Abstraite

Réalisation du projet du cours TAS du Master 2 STL (Sorbonne) de 2016-2017. L'objectif est de se familiariser avec la conception d'un analyseur statique par interprétation abstraite, en Rust (le sujet initial était proposé en OCaml, cf [la documentation](doc/sujet.pdf)).
Le projet sera probablement étoffé au fur à mesure de mes apprentissages.

## Usage

Le projet est écrit en utilisant [cargo](https://doc.rust-lang.org/cargo/) (rustup default stable). Pour build il suffit donc d'exécuter :
```bash
cargo build
```
Puis, pour exécuter le programme, on retrouve le binaire dans `target/debug/rsabsint`, mais il est aussi possible d'utiliser cargo à nouveau :
```bash
cargo run -- [ARGS] fichier.c
```
avec les `ARGS` suivants possibles : `-concrete`, `-constant`, `-interval`, `-disjonctive`, `-unroll n`, `-delay n`.

Pour passer sur la batterie de tests ([dossier des tests](test)), on utilise cargo :
```bash
cargo test
```

## Conteneurisation

Une [image docker](Dockerfile) correspondant au projet a été écrit :
```bash
# construction de l'image docker
docker build -t rsabsint .

# lancement d'un shell dans le container
sudo docker run -it --entrypoint /bin/sh rsabsint
```
au sein du conteneur est copiée la batterie de tests.

## TODO

- [x] Parsing/Lexing du langage.
- [x] Pretty printing de l'arbre de syntaxe abstraite.
- [x] Interprète générique des programmes.
- [ ] domaine concret (option `-concrete`).
- [ ] domaine des constantes (option `-constant`).
- [ ] domaine des intervalles (option `-interval`).
- [x] analyse de boucles.
- [ ] analyse des entiers machine.
- [ ] analyse disjonctive (option `-disjonctive`).
- [ ] analyse relationnelle et bindings Apron.
- [ ] analyse de tableaux.

## Grammaire BNF du langage analysé

Voici la grammaire BNF du langage qui est analysé par `rsabsint` :
```
<file> ::= <stat>* EOF
;
<stat> ::= <block>
         | id EQUAL <int_expr> SEMICOLON
         | IF LPAREN <bool_expr> RPAREN <stat>
         | IF LPAREN <bool_expr> RPAREN <stat> ELSE <stat>
         | WHILE LPAREN <bool_expr> RPAREN <stat>
         | ASSERT LPAREN <bool_expr> RPAREN SEMICOLON
         | PRINT LPAREN <separated_list(COMMA, id)> RPAREN SEMICOLON
         | HALT SEMICOLON
;
<block> ::= LCURLY <decl>* <stat>* RCURLY
;
<decl> ::= <typ> id SEMICOLON
;
<typ> ::= INT_T
;
<int_expr> ::= LPAREN <int_expr> RPAREN
             | INT
             | IDENT
             | <int_unary_op> <int_expr>
             | <int_expr> <int_binary_op> <int_expr>
             | RAND LPAREN <sign_int_literal> COMMA <sign_int_literal> RPAREN
;
<sign_int_literal> ::= INT
                     | PLUS INT
                     | MINUS INT
;
<int_unary_op> ::= PLUS
                 | MINUS
;
<int_binary_op> ::= TIMES
                  | DIV
                  | PLUS
                  | MINUS
                  | MODULO
;
<bool_expr> ::= LPAREN <bool_expr> RPAREN
              | TRUEE
              | FALSEE
              | <bool_unary_op> <bool_expr>
              | <bool_expr> <bool_binary_op> <bool_expr>
              | <int_expr> <compare_op> <int_expr>
;
<bool_unary_op> ::= NOT
;
<bool_binary_op> ::= AND
                   | OR
;
<compare_op> ::= LESS
               | GREATER
               | LESS EQUAL
               | GREATER EQUAL
               | EQUAL EQUAL
               | NOT EQUAL
;
```
