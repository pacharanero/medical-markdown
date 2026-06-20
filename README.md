# Medical Markdown

A very early, partially elaborated idea about a Markdown-like specification 
for quickly writing free text medical notes that can be programmatically 
parsed into structured data, using standard [Academy of Medical Royal Colleges][aomrc]/[Professional Records Standards Body][prsb] [document headings][aomrc-headings], with insertion of clinical coding systems/clinical terminologies where appropriate.

If you haven't previously encountered or used the Markdown system for rapidly formatting HTML documents using plain text delimited by punctuation characters, then I would recommend reading about it [here][dfb-markdown]. An understanding of what Markdown is and how it works is advantageous in order to understand both the similarities and differences of Medical Markdown to 'vanilla' Markdown. In common with Markdown, Medical Markdown is both a **specification** and a series of **reference parser implementations**.

Additionally, the Extended Specification allows for some real-time features, such as selection of codes from a pick list populated in real-time from a REST terminology server. This pick list can be constrained or pre-processed such as to offer the clinician a subset of the available terminology codes, based on that clinicians' commonly used codes, locally recommended codes, or reference codesets for a particular use-case.

As with Markdown, this specification is not intended to be all-encompassing and we will not implement every possible use-case. That way lies madness. A good example is in prescribing of drugs, where (at present at least) the EPR manufacturer's dialog, with a mixture of numeric and text fields, makes more sense and is probably safer to use. Medical Markdown is intended simply to streamline entry of a clinical consultation into an EPR without **text-box overload**, get clinicians' hands off their mice, and get them onto their keyboard where they can get stuff done.

This specification is still evolving, and contributions are welcome. Please see the section below on Contributions.

### Canonical implementation

The Rust crate in this repository is the single canonical implementation of Medical Markdown. The original Python package under `medical-markdown-python/` is retained only as a historical reference and will be removed once nothing depends on it. Behaviour is pinned by the language-agnostic gold-standard conformance suite in [`tests/conformance/`](tests/conformance/README.md): a behaviour is part of the spec when there is a fixture for it. See [`ROADMAP.md`](ROADMAP.md) for the direction, and [`spec.md`](spec.md) for the embedding requirements driving the near-term work (GitEHR is the first real consumer).



## Getting started

### Prerequisites

You need a Rust toolchain installed. The easiest way is via [rustup](https://rustup.rs):

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Quick start

```bash
s/install    # build the project
s/test       # run the test suite
s/dev medical-markdown-python/test_data/simple.txt          # run the CLI (outputs HTML + JSON)
s/dev medical-markdown-python/test_data/simple.txt --json   # JSON structured data only
s/dev medical-markdown-python/test_data/simple.txt --html   # HTML only
echo "PC/ chest pain" | s/dev -                             # read from stdin
```

### All scripts

| Script | Purpose |
|--------|---------|
| `s/install` | Install dependencies and build the project |
| `s/dev` | Run the `medmd` CLI in development mode |
| `s/test` | Run the test suite (`cargo test`) |
| `s/lint` | Run clippy and check formatting |
| `s/build` | Build an optimised release binary to `target/release/medmd` |
| `s/build-wasm` | Build the WASM package to `pkg/` (requires `wasm-pack`) |

### Custom codes

You can extend the built-in clinical codes with a JSON file:

```bash
s/dev input.txt --codes custom-codes.json --json
```

The JSON file should contain an array of code definitions:

```json
[
  { "code": "NEURO", "heading": "Neurological Examination", "category": "Examination" }
]
```

Custom codes are merged with the built-in set. If a custom code has the same abbreviation as a built-in code, the custom heading takes precedence.

### Using as a library (embedding)

Medical Markdown is designed to be embedded as a backend Rust crate dependency (this is how GitEHR consumes it). The one-call entry point handles parser setup so a consumer goes straight from a Markdown body to structured data or HTML:

```rust
let doc = medical_markdown::parse(body);

if doc.has_codes() {
    let data = doc.structured();   // serde_json::Value, empty object for plain prose
    let html = doc.html();         // semantic HTML for display surfaces
}
```

`medical_markdown::has_codes(body)` is a cheap pre-check (no full parse) for deciding whether to offer a structured view at all; most plain-prose bodies answer `false` quickly. To use a custom vocabulary held in memory, build a `CodeRegistry` from a JSON string (or any reader) and pass it by reference, reusing it across calls:

```rust
let registry = medical_markdown::CodeRegistry::from_json_str(codes_json)?;
let doc = medical_markdown::parse_with_registry(body, &registry);
```

Plain-prose bodies with no clinical codes always extract to an empty structure and never error.

### Conformance suite

[`tests/conformance/`](tests/conformance/README.md) holds the gold-standard fixtures (paired `input.md` + `expected.json`) that define the extraction contract independently of any implementation. Run them with:

```bash
cargo test --test conformance
```

### Interactive demo

Open `demo.html` in a browser for a live, in-browser demo of Medical Markdown parsing. If you've run `s/build-wasm`, the demo uses the Rust parser via WASM; otherwise it falls back to a built-in JS parser.

### WASM build

The WASM package lives in a separate workspace crate (`medical-markdown-wasm/`). To build it:

```bash
cargo install wasm-pack    # one-time setup
s/build-wasm               # builds to pkg/
```

Then serve the project directory over HTTP (e.g. `python3 -m http.server`) and open `demo.html`.


## Basic Specification ##

This is the basic specification for Medical Markdown, which specifies only features that could be included in an asynchronous parser (ie no real-time features) and is here:

'Asynchronous parsing' means the text is first written and saved as a text document, and then a parser processes it at some later stage. The key difference is there is no real-time interaction between the parser and the user, making it simpler to implement.

### An example of Basic Medical Markdown

`PMH/`
=> is parsed to the AoMRC heading 'Past Medical History'. The subsequent lines of text are included as structured data under this heading, teminating at the first newline. Where an entry matches *exactly* the rubric (text portion) of an existing terminology, the plain text can be replaced with structured text including the code.

Much of the standard Markdown specification is included in the Medical Markdown specification, where this made sense and didn't interfere with medical/clinical utility.

> Basic Medical Markdown = Daring Fireball Markdown + Medical Markdown bits

Further details are in the medical-markdown Wiki [Medical Markdown Specification][]



## Extended Specification ##

This is the extended specification for Medical Markdown, which includes all of the Basic Specification, and adds some features which can only be offered if the parser is interacting with the user in real-time.

An example of a feature that requires real-time interaction between the user and the parser is interactive selection of a clinical code/terminology from a pick list. This may require any or all of the following data to be processed **synchronously**, in order for the correct set of codes to be shown to the user:

* **Which section of the document is currently being filled in** - for example, in a diagnosis section, one may wish to constrain the available codes to 'diagnosis' codes, in a medication list section, similarly one would constrain the list of options to a local medicines data dictionary.
* **The current user's recent usage of codes** - offering the clinician's most-recently-used or most-commonly-used codes
* **Local, National or International** guidance on appropriate codes for recording a particular piece of clinical information - for example this allows a medical institution to guide their clinicians to recording using an approved code-set.
* **Natural language processing of the free text of the rest of the clinical document** - this may enable an advanced system to intelligently suggest the codes that the clinician might want, based on analysis of the document. But, unlike other suggestions I've heard for automatic-text-to-terminology conversion, *the clinician remains in charge of what is actually entered*.

> Extended Medical Markdown = Basic Medical Markdown + Synchronous Features

Further details are in the medical-markdown Wiki [Medical Markdown Specification][]



## Credits and Acknowledgements ##

* Major credit is of course due to John Gruber of daringfireball.net, for the original 2004 Markdown specification which inspired this work, and has saved me countless hours of point-and-click formatting. Medical Markdown is, like Markdown itself, released under an open source license.

* I'd like to acknowledge the influence of those projects that introduced me to the wonders of Markdown and that have gradually spread the utility of markdown all over the web: [GitHub][], [Discourse][], [Jekyll][], [Trello][], [Prose.io][], and many others.

* I'd also like to credit Gina Trapani, creator of [Todo.txt][], whose inspiration I have also used in the way I've handled the `TODO/`` 'jobs list' tag.

* The Professional Records Standards Body deserve due acknowledgement for the work done on developing clear standards for medical records in the UK, and the Academy of Medical Royal Colleges and it's constituent colleges are recognised for adding their endorsement to the medical record content standards.



## Contributing ##

Here are some ways to contribute to the Medical Markdown project:

* Talk about the project with your colleagues and see what they think.
* Submit improvement suggestions, problems, bugs, or just plain verbal abuse as GitHub Issues (see [here][gh-create-issue] for a guide on how to do this, and [here](mmd-issues) for our Issues section)
* Talk about Medical Markdown with your CCIO, CNIO, or even the vendor of your EPR!
* Donate to the project, to allow me to spend more time developing it.
* Spread the word via social media
* If the AoMRC headings make sense in your country's flavour of medicine, you could to translate Medical Markdown into your language.



[aomrc]: http://www.aomrc.org.uk
[prsb]: http://theprsb.org
[aomrc-headings]: http://theprsb.org/publications/bible-sets-out-the-latest-agreed-standards
[dfb-markdown]: http://daringfireball.net/projects/markdown/
[Medical Markdown Specification]: https://github.com/open-health-hub/medical-markdown/wiki
[GitHub]: http://www.github.com
[Discourse]: http://www.discourse.org
[Jekyll]: http://www.jekyllrb.com
[Trello]: http://www.trello.com
[Prose.io]: http://prose.io/
[Todo.txt]: http://todotxt.com/
[gh-create-issue]: http://help.github.com/articles/creating-an-issue/
[mmd-issues]: https://github.com/open-health-hub/medical-markdown/issues
