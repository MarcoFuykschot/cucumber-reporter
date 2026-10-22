
#import "scenario.typ": *
#import "styles.typ": *

#let render_outline(scenario) = {

   show regex("<[^>]+>"): placeholder => {
    show "<": ""
    show ">": ""
    text(style: "italic", fill: gray)[#placeholder]
  }

  render_scenario(scenario)

  [
    #for example in scenario.examples [
      #emph([Examples])
      #render_tags(example.tags)
      #table(
        columns: example.columns,
        stroke: table_stroke,
        ..example.rows.enumerate().map(((index, row)) => row.map(cell => {
          if index == 0 {
            text(weight: "bold")[#outcome(cell)]
          } else {
            outcome(cell)
          }
        })).flatten()
      )
    ]
  ]
}
