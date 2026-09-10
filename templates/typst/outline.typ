
#import "scenario.typ": *

#let render_outline(scenario) = {

   show regex("<[^>]+>"): placeholder => {
    show "<": ""
    show ">": ""
    text(style: "italic", fill: gray)[#placeholder]
  }

  render_scenario(scenario)

  [
    #for example in scenario.examples [
      === Examples
      #table(
        columns: example.columns,
        ..example.rows.map(row => row.map(cell => [#cell])).flatten()
      )
    ]
  ]
}
