#import sys: inputs, version
#import "@preview/in-dexter:0.7.2": *
#import "feature.typ": *
#import "styles.typ": render_markdown, render_table_header

#let runinfo = inputs.at("runinfo", default: {})
#let is-html = inputs.at("is_html", default: false)
#let render_runinfo_content(field) = {
  let content = runinfo.at(field, default: none)
  let format = runinfo.at(field + "_format", default: "markdown")
  if content == none {
    []
  } else if format == "typst" {
    eval(content, mode: "markup")
  } else {
    render_markdown(content)
  }
}
#let render_page_content(field) = context {
  let page_number = counter(page).get().first()
  if page_number > 1 { render_runinfo_content(field) } else { [] }
}

#set page(
  paper: "a4",
  margin: (top: 1.5cm, bottom: 1.5cm, left: 1.2cm, right: 1.2cm),
  header: if is-html { [] } else { render_page_content("header") },
  footer: if is-html { [] } else { render_page_content("footer") },
)

#set text(font: "New Computer Modern", size: 11pt)
#set heading(numbering: "1.")

#show table.cell: set text(hyphenate: true)

#if runinfo != none {
  set document(
    title: [#runinfo.title],
    author: runinfo.author,
  )

  let title-page = runinfo.at("title_page", default: none)
  if title-page == none {
    align(center)[
      #title([#runinfo.title])
      #runinfo.author
    ]
  } else if runinfo.at("title_page_format", default: "markdown") == "typst" {
    eval(title-page, mode: "markup")
  } else {
    render_markdown(title-page)
  }

  pagebreak()
}

#outline()
#pagebreak()

#if is-html and runinfo.at("header", default: none) != none [
  #render_runinfo_content("header")
]

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

#if is-html and runinfo.at("footer", default: none) != none [
  #render_runinfo_content("footer")
]