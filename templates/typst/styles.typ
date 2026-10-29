#import "@preview/cmarker:0.1.10"
#import "@preview/in-dexter:0.7.2": *
#import sys: inputs

#let table_stroke = (paint: luma(200), thickness: 0.4pt)

#let table_header_rotation(headers, columns) = {
  if inputs.at("is_html", default: false) or columns <= 0 {
    none
  } else {
    let max_column_width = 18cm / columns - 20pt
    let needs_rotation = headers.any(
      header => measure(text(weight: "bold")[#header]).width > max_column_width,
    )
    if not needs_rotation {
      none
    } else {
      let fitting_angles = range(65, 91).filter(angle =>
        headers.all(header =>
          {
            let size = measure(text(weight: "bold")[#header])
            let rotated_width = size.width * calc.cos(angle * 1deg) + size.height * calc.sin(angle * 1deg)
            rotated_width <= max_column_width
          },
        ),
      )
      if fitting_angles.len() > 0 {
        fitting_angles.first() * 1deg
      } else {
        90deg
      }
    }
  }
}

#let render_table_header(cell, rotation: none) = {
  let header = text(weight: "bold")[#cell]
  if rotation != none {
    align(bottom + center)[#rotate(-rotation, reflow: true)[#header]]
  } else {
    header
  }
}

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
