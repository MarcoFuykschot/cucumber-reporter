#import "styles.typ": *

#let render_step(step) = {
  [
    #show regex("Given|When|Then"): keyword => box(
      width: 45pt,
      text(fill: blue, weight: "bold")[#keyword:],
    )
    #show regex("And|But"): keyword => box(
      inset: (left: 10pt),
      width: 45pt,
      text(fill: blue, weight: "bold")[#keyword:],
    )
    #step.keyword
    #step.text
    #outcome([#step.outcome]) \
  ]
  if step.error_message != none [
    #box(
      inset: (left: 2em, top: 0.2em),
      text(size: 0.8em, fill: red)[Error: #step.error_message],
    )
  ]
  if step.table != none {
    let tab = step.table
    let rows = tab.rows
    let headers = rows.at(0, default: ())
    box(inset: (left: 10pt), [
      #context {
        let header_rotation = table_header_rotation(headers, tab.columns)
        table(
          columns: table_columns(tab.columns),
          stroke: table_stroke,
          ..rows.enumerate().map(((index, row)) => row.map(cell => {
            if index == 0 {
              render_table_header(cell, rotation: header_rotation)
            } else {
              cell
            }
          })).flatten()
        )
      }
    ])
    [ \ ]
  }
  if step.doc_string != none [
    #render_description(step.doc_string) \
  ]
}

#let render_scenario(scenario) = [
  ==== Scenario: #scenario.name

  #render_tags(scenario.tags)

  #render_description(scenario.description)

  #if scenario.steps.len() > 0 [
    #for step in scenario.steps {
      render_step(step)
    }
  ] else [
    #emph[No steps defined for this scenario.]
  ]
]
