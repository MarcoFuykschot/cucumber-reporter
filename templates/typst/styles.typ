#import "@preview/cmarker:0.1.10"
#import "@preview/in-dexter:0.7.2": *

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
        inset: (x: 5pt, y: 3pt),
      )[
        #tag #index[#tag]
      ])
      .join(h(5pt))
    parbreak()
  }
}

#let render_description(description) = {
  if description != none {
    box(inset: (left: 10pt), [
      #context {
        set heading(bookmarked: false, outlined: false, numbering: none)
        show heading: set text(weight: "bold", size: 8pt)
        show heading.where(level: 1): set text(size: 10pt)
        show heading.where(level: 2): set text(size: 9pt)
        cmarker.render(description)
      }
    ])
  }
}
