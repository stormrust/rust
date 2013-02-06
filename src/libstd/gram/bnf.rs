mod bnf {

    /*
    
    <syntax> ::= <rule> | <rule> <syntax>
    <rule>   ::= <opt-whitespace> "<" <rule-name> ">" <opt-whitespace> "::=" 
                 <opt-whitespace> <expression> <line-end>
    <opt-whitespace> ::= " " <opt-whitespace> | ""  ; "" is empty string, i.e. no whitespace
    <expression>     ::= <list> | <list> "|" <expression>
    <line-end>       ::= <opt-whitespace> <EOL> | <line-end> <line-end>
    <list>    ::= <term> | <term> <opt-whitespace> <list>
    <term>    ::= <literal> | "<" <rule-name> ">"
    <literal> ::= '"' <text> '"' | "'" <text> "'" ; actually, the original BNF did not use quotes

    */

    struct Gram {
        rules: ~[Rule]
    }

    struct Rule {
        name: ~str,
        expr: ~[List]
    }

    struct List {
        syms: ~[Symbol]
    }

    enum Symbol {
        Terminal(~str),
        NonTerminal(~str)
    }

}
