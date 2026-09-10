#import sys: inputs, version

#set page(
  paper: "a4",
  margin: (top: 2cm, bottom: 2cm, left: 1.5cm, right: 1.5cm),
)

#set text(font: "New Computer Modern", size: 11pt)
#set heading(numbering: "1.")

#import "feature.typ": *

#outline()
#pagebreak()

#let features = inputs.at("features", default: ())
#for (index, item) in features.enumerate() [
  #feature(item)
  #if index < features.len() - 1 [
    #pagebreak()
  ]
]