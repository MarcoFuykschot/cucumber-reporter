
#import "scenario.typ": *
#import "styles.typ": *

#show table.cell: set text(hyphenate: true)

#let render_outline(scenario, level: 3) = {

   show regex("<[^>]+>"): placeholder => {
    show "<": ""
    show ">": ""
    text(style: "italic", fill: gray)[#placeholder]
  }

  render_scenario(scenario, level: level, show_step_outcome: false)

  if scenario.examples.len() > 0 [
    #render_section_label("Variations")
    #for example in scenario.examples [
      #if example.name != none [
        #text(weight: "bold", fill: gray.darken(30%))[#example.name]
        #v(4pt)
      ]

      #if example.tags.len() > 0 [
        #render_tags(example.tags)
      ]

      #let rows = example.rows
      #context {
        table(
          columns: table_columns(example.columns),
          stroke: table_stroke,
          inset: 4pt,
          ..rows.enumerate().map(((index, row)) => row.map(cell => {
            if index == 0 {
              render_table_header(outcome(cell))
            } else {
              outcome(cell)
            }
          })).flatten()
        )
      }
      #v(6pt)
    ]
  ]
}
