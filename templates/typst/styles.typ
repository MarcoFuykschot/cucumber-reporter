#import "@preview/cmarker:0.1.10"
#import "@preview/in-dexter:0.7.2": *

#let table_stroke = (paint: luma(200), thickness: 0.4pt)

#let render_markdown(markdown) = cmarker.render(markdown)

#let render_table_header(cell) = text(weight: "bold", hyphenate: true)[#cell]

#let table_columns(count) = range(count).map(_ => 1fr)

#let outcome(value) = {
  show "passed": outcome => text(fill: green, weight: "bold")[#outcome]
  show "failed": outcome => text(fill: red, weight: "bold")[#outcome]
  show "skipped": outcome => text(fill: gray, weight: "bold")[#outcome]
  [#value]
}

#let render_tags(tags) = {
  if tags.len() > 0 {
    tags
      .map(tag => box(
        fill: gray,
        inset: (x: 4pt, y: 2pt),
      )[
        #tag #index[#tag]
      ])
      .join(h(3pt))
    parbreak()
  }
}

#let render_description(description) = {
  if description != none {
    v(0.3em)
    block(inset: (left: 6pt), [
      #context {
        set heading(bookmarked: false, outlined: false, numbering: none)
        show heading: set text(weight: "bold", size: 8pt)
        show heading.where(level: 1): set text(size: 10pt)
        show heading.where(level: 2): set text(size: 9pt)
        cmarker.render(description)
      }
    ])
    v(0.3em)
  }
}

#let render_section_label(title) = {
  text(fill: gray.darken(30%), weight: "bold", size: 0.8em)[#title:]
  v(2pt)
}
