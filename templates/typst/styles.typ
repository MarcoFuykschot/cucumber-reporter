#import "@preview/cmarker:0.1.10"

#let outcome(value) = {
  show "passed": outcome => text(fill: green, weight: "bold")[#outcome]
  show "failed": outcome => text(fill: red, weight: "bold")[#outcome]
  show "skipped": outcome => text(fill: gray, weight: "bold")[#outcome]
  [#value]
}

#let render_tags(tags) = {
  if tags.len() > 0 [
    Tags: #tags.join(" ") \
  ]
}

#let render_description(description) = {
  if description != none {
    box(inset: (left: 10pt), [
      #set heading(bookmarked: false, numbering: none, outlined: false)
      #show heading: it => {
        let rendered = if it.level == 1 {
          set text(weight: "bold", size: 10pt)
          it.body
        } else if it.level == 2 {
          set text(weight: "bold", size: 9pt)
          it.body
        } else {
          set text(weight: "bold", size: 8pt)
          it.body
        }
        rendered
        parbreak()
      }
      #counter(heading).update(0)
      #cmarker.render(description)
      #cmarker.render-with-metadata(description)
      #raw(description,block: true)
    ])
  }
}
