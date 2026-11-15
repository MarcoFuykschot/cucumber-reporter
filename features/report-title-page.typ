#import sys: inputs

#let is-html = inputs.at("is_html", default: false)
#let cover = [
  #strong[Typst title page example]

  Gherkin feature report

  Version 1.0
]

#if is-html {
  cover
} else {
  v(5cm)
  align(center)[
    #set text(size: 24pt)
    #cover
  ]
}

#pagebreak()

#heading(numbering: none, outlined: false)[Document history]

#table(
  columns: (1fr, auto, auto, 2fr),
  inset: 8pt,
  table.header(
    [*Author*], [*Version*], [*Date*], [*Changes*],
  ),
  [Example author], [0.1], [2026-11-01], [Initial draft],
  [Example reviewer], [1.0], [2026-11-15], [Reviewed and approved for publication],
)
