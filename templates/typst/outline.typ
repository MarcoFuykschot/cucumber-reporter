
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
      #let rows = example.rows
      #let headers = rows.at(0, default: ())
      #context {
        let header_rotation = table_header_rotation(headers, example.columns)
        table(
          columns: table_columns(example.columns),
          stroke: table_stroke,
          ..rows.enumerate().map(((index, row)) => row.map(cell => {
            if index == 0 {
              render_table_header(outcome(cell), rotation: header_rotation)
            } else {
              outcome(cell)
            }
          })).flatten()
        )
      }
    ]
  ]
}
