#import sys: inputs, version
#import "@preview/in-dexter:0.7.2": *

#set page(
  paper: "a4",
  margin: (top: 1.5cm, bottom: 1.5cm, left: 1.2cm, right: 1.2cm),
)

#set text(font: "New Computer Modern", size: 11pt)
#set heading(numbering: "1.")

#import "feature.typ": *
#import "styles.typ": render_markdown, render_table_header

#show table.cell: set text(hyphenate: true)

#let runinfo = inputs.at("runinfo", default: {})

#if runinfo != none {
  set document(
    title: [#runinfo.title],
    author: runinfo.author,
  )

  align(center)[
    #title([#runinfo.title])
    #runinfo.author
  ]

  pagebreak()
}

#outline()
#pagebreak()

#let features = inputs.at("features", default: ())

= Features

#let feature-introduction = runinfo.at("feature_introduction", default: none)
#if feature-introduction != none [
  #if runinfo.feature_introduction_format == "markdown" [
    #render_markdown(feature-introduction)
  ] else if runinfo.feature_introduction_format == "typst" [
    #eval(feature-introduction, mode: "markup")
  ] else [
    #feature-introduction
  ]
]

#if features != none and features.len() > 0 [
  #let summary_rows = features.enumerate().map(((index, item)) => {
    let feature-id = "feature-" + str(index)
    (
      [#link(label(feature-id))[#item.name]],
      if item.outcome != none {
        [#outcome(item.outcome)]
      } else {
        []
      },
    )
  })
  #table(
    columns: (1fr, auto),
    inset: 4pt,
    align: left,
    stroke: table_stroke,
    render_table_header([Feature]), render_table_header([Result]),
    ..summary_rows.flatten(),
  )
  #pagebreak()
]

#if features != none {
  for (index, item) in features.enumerate() [
    #render_feature(item, index: index)
    #if index < features.len() - 1 [
      #pagebreak()
    ]
  ]
}

#pagebreak()
= Tag index
#columns(2)[
  #make-index(title: none,outlined: true )
]