# EXAMPLE

``` 
EXT {
    pdf,doc,docx: EXT {
        pdf: 'Documents/PDF'
        doc,docx: 'Documents/Word'
    }
    py,pyc: Code
    c,cpp,h,hpp: Code
    java: Code
    js: Code
    html,htm: Code
    jpg,jpeg,png,gif: Images
    mp3: Music
    mp4,mkv,avi: Videos
}
```

``` 
RULE_TYPE {
    COND: RULE_TYPE {
        COND: RULE
        RULE: RULE
    }
}
```

# Lexical Grammar

```
Identifier -> Letter {Letter | Digit | '_'}*
Symbol -> : | , | { | } | ' | " 
String -> Symbol::Quote {Letter | Digit | ' '}* Symbol::Quote
Letter -> 'a'..'z' | 'A'..'Z'
Digit -> '0'..'9'
```