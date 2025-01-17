
# NQL

**NQL** - Neutral Query Language. This project is a very basic translator that skips *syntax* and *semanthic* *analysises*. Basically, NQL translator takes source text written in NQL. Then, it parses it into so called tokens and classifies them. Finally, it translates NQL tokens into another language tokens.


## Documentation on language grammar

NQLang is the main lexeme. Basicaly, NQLang means a valid sequance of other lexemes
that can represent NQL.
```
NQLang ::= UnitsSequance Extension*
```

UnitsSequance is the main part of the language that contains statements and group
of statements that are connected with the link.
```
UnitsSequance ::= Unit (Link Unit)*
```

Link is the boolean operator that establishes logical relations between language's
units.
```
Link ::= WhitespaceWrap (AND | OR) WhitespaceWrap
AND ::= \&
OR ::= \|
```

Unit is either a statement or a group wrapped with whitespaces.
```
Unit ::= WhitespaceWrap (Statement | Group) WhitespaceWrap
```

Group is valid sequance of NQL's lexemes wrapped with braces.
```
Group ::= OpenBrace UnitsSequance CloseBrace
OpenBrace ::= \(
CloseBrace ::= \)
```

Statement is just a pair of key and value.
```
Statement ::= KeyValue
```

Extensions is the optional part of the language, but some converters require
them in order to convert NQL tokens to their own. Like for example SQL converter
requires extension called 'table' and if it is not provided, then it will panic.
```
Extension ::= ExtensionEscape KeyValue WhitespaceWrap
ExtensionEscape ::= \$
```

```
KeyValue ::= Key ComparasionOperator Value
ComparasionOperator ::= WhitespaceWrap (= | (!=) | (<=) | (>=) | < | >) WhitespaceWrap

Key ::= Ident(Point Ident)*
Ident ::= [a-zA-Z_][a-zA-Z0-9_]*

Value ::= OrdinaryValue | Range | Collection
```

```
OrdinaryValue ::= Boolean | Date | Number | String
```

```
Boolean ::= True | False
True ::= [Tt][Rr][Uu][Ee]
False ::= [Ff][Aa][Ll][Ss][Ee]
```

Dates are also checked for corrletion of number of days to a month number, leap
years are alse checked.
```
Date ::= Day:Month:Year
Day ::= Integer
    WARN!!! 1 <= Day <= 31
Month ::= Integer
    WARN!!! 1 <= Month <= 12
Year ::= Integer
```

```
Number ::= Integer | Float
Integer ::= Minus? Digit+
Float ::= Integer Point Digit+
Digit ::= [0-9]
Minus ::= -
Point ::= \.
```

```
String ::= RegularString | SingleQuotedString | DoubleQuotedString

RegularString ::= [^Whitespace ReservedChars]+
Whitespace -> Any Non Visible Character

ReservedChars ::= OpenBrace | CloseBrace | Comma  | And | Or |  
    ACOpenBrace |  ACCloseBrace |  OCOpenBrace |  OCCloseBrace |  
    DoubleQuote | SingleQuote |  EscapeChar
Comma ::= ,
ACOpenBrace ::= {
ACCloseBrace ::= }
OCOpenBrace ::= [
OCCloseBrace ::= ]
DoubleQuote ::= "
SingleQuote ::= '
EscapeChar ::= \\

SingleQuotedString ::= SingleQuote [^SingleQuote]* SingleQuote
DoubleQuotedString ::= DoubleQuote [^DoubleQuote]* DoubleQuote
```

```
Range := DateRange | NumberRange
DateRange ::= Date RangeOp Date
NumberRange ::= Number RangeOp Number

RangeOp ::= WhitespaceWrap (EE | IE | EI | II) WhitespaceWrap
EE ::= Point Point
IE ::= EqSign Point
EI ::= Point EqSign
II := EqSign EqSign
E -> exclusively
I -> inclusively
```

```
Collection ::= AndCollection | OrCollection
AndCollection ::= ACOpenBrace WhitespaceWrap CollectionBody WhitespaceWrap ACCloseBrace
OrCollection ::= OCOpenBrace WhitespaceWrap CollectionBody WhitespaceWrap OCCloseBrace
CollectionBody ::= OrdinaryValue (WhitespaceWrap Comma WhitespaceWrap OrdinaryValue)*
```

```
WhitespaceWrap ::= Whitespace*
Whitespace -> Any Non Visible Character
```

## FAQ

#### What languages NQL supports?

NQL translator currently supports next list:
- SQL;
- Lucene;
- WQL (*Wazuh Query Language*).

#### Can I translate language from one of the supported ones into NQL?

No! Translator can only parse NQL tokens and translate them into another language tokens. And that's all.

#### What are extensions? I don't understand why NQL has them.

NQL was created to unify all the query languages it supports. So on the one hand it solves the problem of the user having to learn whole set of languages by introducing one common language. However, on the other hand it is not possible to bring all languages together in one. Here comes the extensions. They are used in building specifical cases like sql table name and so on. But they are not used in query.

#### I am a developer, that will support this project. What basics should i know?

You should read about [parser combinators](https://en.wikipedia.org/wiki/Parser_combinator). If you better with practical guides, then is is right [here](https://bodil.lol/parser-combinators/).
If you will extend supported languages, then all you should know about is NQL grammar, that is specified in documentation, and and see a code of nql::lang::to_sql for example. But no matter, what you are doing in the project, if you introduce some new features, please, dont be lazy and write tests.

## Running Tests

To run tests, run the following command

```bash
  cargo test
```

