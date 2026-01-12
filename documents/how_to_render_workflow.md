# How to render workflow

## Task Description

## Learning to Write Parsers with Nom

Nom is a parser combinator library for Rust that allows you to build complex parsers by combining simple ones. Here's how to learn and implement parsers using Nom:

### 1. Understanding Parser Combinators

Parser combinators are higher-order functions that take parsers as input and return a new parser as output. In Nom:
- A parser is a function that takes an input and returns a result
- Combinators allow you to chain, combine, and transform parsers

### 2. Basic Nom Concepts

```rust
use nom::{
    IResult, // Result type: (remaining_input, parsed_output)
    bytes::complete::{tag, take_until},
    character::complete::{char, alpha1, digit1, space0, space1},
    combinator::{opt, map},
    sequence::{preceded, terminated, tuple},
    multi::{many0, many1},
    branch::alt,
};

// Basic parser that recognizes the string "hello"
fn parse_hello(input: &str) -> IResult<&str, &str> {
    tag("hello")(input)
}

// Parser that recognizes a word (sequence of alphabetic characters)
fn parse_word(input: &str) -> IResult<&str, &str> {
    alpha1(input)
}

// Parser that recognizes a number
fn parse_number(input: &str) -> IResult<&str, &str> {
    digit1(input)
}
```

### 3. Common Nom Combinators

- `tag("literal")` - Matches a literal string
- `alpha1()` - Matches one or more alphabetic characters
- `digit1()` - Matches one or more digits
- `space0()` - Matches zero or more whitespace characters
- `space1()` - Matches one or more whitespace characters
- `opt(parser)` - Makes a parser optional
- `many0(parser)` - Applies parser zero or more times
- `many1(parser)` - Applies parser one or more times
- `alt((parser1, parser2))` - Tries multiple parsers in sequence
- `preceded(delimiter, parser)` - Parses delimiter then parser
- `terminated(parser, delimiter)` - Parses parser then delimiter
- `tuple((parser1, parser2))` - Applies multiple parsers in sequence

### 4. Building a Simple YAML-like Parser

For your workflow files, you need to parse structures like:

```yaml
id: workflow-id
name: Workflow Name
graphs:
  - !include path/to/file.yml
  - id: graph-id
    name: Graph Name
    nodes:
      - id: node-id
        name: Node Name
        type: action
```

Here's how to build a parser for this:

```rust
use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::{alpha1, char, digit1, multispace0, newline, space0, space1},
    combinator::{map, opt, recognize},
    multi::{many0, many1},
    sequence::{delimited, preceded, separated_pair, tuple},
    IResult,
};

// Parse a simple key-value pair like "id: value"
fn parse_key_value(input: &str) -> IResult<&str, (&str, &str)> {
    let (input, key) = alpha1(input)?;
    let (input, _) = preceded(space0, tag(":"))(input)?;
    let (input, _) = space1(input)?;
    let (input, value) = parse_value(input)?;

    Ok((input, (key, value)))
}

// Parse a value (handling quoted strings and unquoted strings)
fn parse_value(input: &str) -> IResult<&str, &str> {
    alt((
        // Quoted strings
        delimited(char('"'), take_until("\""), char('"')),
        delimited(char('\''), take_until("'"), char('\'')),
        // Unquoted strings (until newline or comment)
        recognize(many1(none_of("\n# \t")))
    ))(input)
}

// Parse a list element that might be an include directive or inline definition
fn parse_list_element(input: &str) -> IResult<&str, &str> {
    let (input, _) = char('-')(input)?;
    let (input, _) = space1(input)?;

    // Either an include directive or an inline definition
    let (input, content) = alt((
        preceded(tag("!include "), take_until("\n")),
        take_until("\n")
    ))(input)?;

    Ok((input, content))
}
```

### 5. Handling Custom Tags Like `!include`

Custom tags like `!include` require special handling. You can create a parser that recognizes these tags and processes them separately:

```rust
use std::path::PathBuf;

fn parse_include_directive(input: &str) -> IResult<&str, &str> {
    preceded(
        tuple((space0, tag("!include"), space1)),
        take_until("\n")
    )(input)
}

// Recursive inclusion handler
fn process_includes(content: &str, base_path: &PathBuf) -> Result<String, Box<dyn std::error::Error>> {
    // Use regex to find all !include directives
    let include_regex = regex::Regex::new(r"!\s*include\s+([^\n]+)")?;

    let mut result = content.to_string();
    let parent_dir = base_path.parent().unwrap_or(std::path::Path::new("."));

    for cap in include_regex.captures_iter(content) {
        let full_match = &cap[0];
        let file_path_str = cap[1].trim();

        let included_path = parent_dir.join(file_path_str);
        let included_content = std::fs::read_to_string(&included_path)?;

        // Recursively process includes in the included file
        let processed_included_content = process_includes(&included_content, &included_path)?;

        result = result.replace(full_match, &processed_included_content);
    }

    Ok(result)
}
```

### 6. Complete Workflow Parser Structure

Here's a skeleton for a complete workflow parser:

```rust
use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::{alpha1, char, multispace0, newline, space0, space1},
    combinator::{opt},
    multi::{many0},
    sequence::{preceded, tuple},
    IResult,
};

#[derive(Debug)]
struct WorkflowDef {
    id: String,
    name: String,
    entry_graph_id: Option<String>,
    graphs: Vec<GraphDef>,
}

#[derive(Debug)]
struct GraphDef {
    id: String,
    name: String,
    nodes: Vec<NodeDef>,
}

#[derive(Debug)]
struct NodeDef {
    id: String,
    name: String,
    node_type: String,
    action: Option<String>,
}

fn parse_workflow(input: &str) -> IResult<&str, WorkflowDef> {
    let (input, _) = multispace0(input)?;

    // Parse id
    let (input, _) = tag("id:")(input)?;
    let (input, _) = space1(input)?;
    let (input, id) = parse_value(input)?;
    let (input, _) = multispace0(input)?;

    // Parse name
    let (input, _) = tag("name:")(input)?;
    let (input, _) = space1(input)?;
    let (input, name) = parse_value(input)?;
    let (input, _) = multispace0(input)?;

    // Parse optional entryGraphId
    let (input, entry_graph_id) = opt(|i| {
        let (i, _) = tag("entryGraphId:")(i)?;
        let (i, _) = space1(i)?;
        parse_value(i)
    })(input)?;
    let (input, _) = multispace0(input)?;

    // Parse graphs section
    let (input, _) = tag("graphs:")(input)?;
    let (input, graphs) = preceded(
        multispace0,
        many0(preceded(space0, parse_graph_element))
    )(input)?;

    Ok((input, WorkflowDef {
        id: id.to_string(),
        name: name.to_string(),
        entry_graph_id: entry_graph_id.map(|s| s.to_string()),
        graphs,
    }))
}

fn parse_graph_element(input: &str) -> IResult<&str, GraphDef> {
    let (input, _) = char('-')(input)?;

    // Either an include directive or inline graph definition
    let (input, graph) = alt((
        parse_include_directive_as_graph,
        parse_inline_graph
    ))(input)?;

    Ok((input, graph))
}

// Continue with implementations for other parsing functions...
```

### 7. Best Practices

1. **Start Simple**: Begin with parsing basic key-value pairs before moving to complex structures
2. **Handle Errors Gracefully**: Use Nom's error handling capabilities
3. **Test Incrementally**: Test each parser function individually
4. **Consider Indentation**: YAML is indentation-sensitive, so preserve formatting when processing includes
5. **Recursive Processing**: Handle nested includes properly

### 8. Resources for Learning More

- Nom documentation: https://docs.rs/nom/
- Nom tutorial: https://github.com/Geal/nom/blob/main/doc/choosing_a_combinator.md
- Nom examples: Look for YAML/JSON parser examples in the Nom repository
- Practice: Start with simple grammars and gradually increase complexity



## References 

- [Parse and Generate YAML with Rust](https://mojoauth.com/parse-and-generate-formats/parse-and-generate-yaml-with-rust/#handling-dynamic-or-unknown-yaml-structures)
- [Learning Parser Combinators With Rust](https://bodil.lol/parser-combinators/)
- [https://iximiuz.com/en/posts/rust-writing-parsers-with-nom/](https://iximiuz.com/en/posts/rust-writing-parsers-with-nom/)