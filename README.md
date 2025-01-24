# Rust-AbsInt : Analyseur statique par Interprétation Abstraite

Réalisation du projet du cours TAS du Master 2 STL (Sorbonne) de 2016-2017. L'objectif est de se familiariser avec la conception d'un analyseur statique par interprétation abstraite, en Rust (le sujet initial était proposé en OCaml, cf [la documentation](doc/sujet.pdf)).
Le projet sera probablement étoffé au fur à mesure de mes apprentissages.

## Usage

Le projet est écrit en utilisant [cargo](https://doc.rust-lang.org/cargo/) (rustup default stable). Pour build il suffit donc d'exécuter :
```bash
cargo build
```
Puis, pour exécuter le programme, on retrouve le binaire dans `target/debug/rust-absint`, mais il est aussi possible d'utiliser cargo à nouveau :
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
docker build -t rust-absint .

# lancement d'un shell dans le container
sudo docker run -it --entrypoint /bin/sh rust-absint
```
au sein du conteneur est copiée la batterie de tests.

## TODO

- [ ] Parsing/Lexing du langage.
- [ ] Pretty printing de l'arbre de syntaxe abstraite.
- [ ] Interprète générique des programmes.
- [ ] domaine concret (option `-concrete`).
- [ ] domaine des constantes (option `-constant`).
- [ ] domaine des intervalles (option `-interval`).
- [ ] analyse de boucles.
- [ ] analyse des entiers machine.
- [ ] analyse disjonctive (option `-disjonctive`).
- [ ] analyse relationnelle et bindings Apron.
- [ ] analyse de tableaux.

## Grammaire BNF du langage analysé

Voici la grammaire BNF du langage qui est analysé par `rust-absint` :
```
<file> ::= <stat>* TOK_EOF
;
<stat> ::= <block>
         | TOK_id TOK_EQUAL <int_expr> TOK_SEMICOLON
         | TOK_IF TOK_LPAREN <bool_expr> TOK_RPAREN <stat>
         | TOK_IF TOK_LPAREN <bool_expr> TOK_RPAREN <stat> TOK_ELSE <stat>
         | TOK_WHILE TOK_LPAREN <bool_expr> TOK_RPAREN <stat>
         | TOK_ASSERT TOK_LPAREN <bool_expr> TOK_RPAREN TOK_SEMICOLON
         | TOK_PRINT TOK_LPAREN <separated_list(TOK_COMMA, TOK_id)> TOK_RPAREN TOK_SEMICOLON
         | TOK_HALT TOK_SEMICOLON
;
<block> ::= TOK_LCURLY <decl>* <stat>* TOK_RCURLY
;
<decl> ::= <typ> TOK_id TOK_SEMICOLON
;
<typ> ::= TOK_INT
;
<int_expr> ::= TOK_LPAREN <int_expr> TOK_RPAREN
             | TOK_int
             | TOK_id
             | <int_unary_op> <int_expr>
             | <int_expr> <int_binary_op> <int_expr>
             | TOK_RAND TOK_LPAREN <sign_int_literal> TOK_COMMA <sign_int_literal> TOK_RPAREN
;
<sign_int_literal> ::= TOK_int
                     | TOK_PLUS TOK_int
                     | TOK_MINUS TOK_int
;
<int_unary_op> ::= TOK_PLUS
                 | TOK_MINUS
;
<int_binary_op> ::= TOK_TIMES
                  | TOK_DIV
                  | TOK_PLUS
                  | TOK_MINUS
                  | TOK_MODULO
;
<bool_expr> ::= TOK_LPAREN <bool_expr> TOK_RPAREN
              | TOK_TRUE
              | TOK_FALSE
              | <bool_unary_op> <bool_expr>
              | <bool_expr> <bool_binary_op> <bool_expr>
              | <int_expr> <compare_op> <int_expr>
;
<bool_unary_op> ::= TOK_EXCLAIM
;
<bool_binary_op> ::= TOK_AND_AND
                   | TOK_BAR_BAR
;
<compare_op> ::= TOK_LESS
               | TOK_GREATER
               | TOK_LESS_EQUAL
               | TOK_GREATER_EQUAL
               | TOK_EQUAL_EQUAL
               | TOK_NOT_EQUAL
;
```
