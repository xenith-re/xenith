# Documentation

This documentation tries to follow the [Diátaxis](https://diataxis.fr/) documentation structure, which is a systematic approach to technical documentation authoring.

<img class="center" src="https://diataxis.fr/_images/diataxis.png" alt="Diátaxis documentation structure"/>

Documentation isn’t just one thing, it is made up of four different parts which include — Tutorials, How-to guides, Explanations, and References.

```admonish tip
For more information, you can visit the [Diátaxis website](https://diataxis.fr/) or read the [Diátaxis documentation](https://diataxis.fr/start-here/). Also, there is this good article [How to Structure Documentation using the Diataxis Framework](https://medium.com/@techwritershub/how-to-structure-documentation-using-the-diataxis-framework-70d4a5a61db7) from Tech Writers Hub Medium blog.
```

## mdBook pre-processors

This documentation is built using [mdBook](https://github.com/rust-lang/mdBook), which allows us to use pre-processors to extend the markdown syntax. We use the following pre-processors:

- [mdbook-mermaid](https://github.com/badboy/mdbook-mermaid) : to render diagrams using the [Mermaid](http://mermaid.js.org/) syntax.
- [mdbook-admonish](https://github.com/tommilligan/mdbook-admonish) : to render admonitions (note, warning, tip, important, caution, etc.), see [reference](https://tommilligan.github.io/mdbook-admonish/reference.html).
