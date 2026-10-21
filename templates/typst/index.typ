#import sys: inputs, version
#import "@preview/in-dexter:0.7.2": *

#set page(
  paper: "a4",
  margin: (top: 2cm, bottom: 2cm, left: 1.5cm, right: 1.5cm),
)

#set text(font: "New Computer Modern", size: 11pt)
#set heading(numbering: "1.")

#import "feature.typ": *

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
    inset: 7pt,
    align: left,
    stroke: table_stroke,
    [Feature], [Outcome],
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
#columns(3)[
  #make-index(title: none,outlined: true , section-title: (_,_) => [])
]