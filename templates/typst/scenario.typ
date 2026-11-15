#import "styles.typ": *

#let render_step_details(step) = {
  if step.error_message != none [
    #linebreak()
    #box(
      inset: (left: 2em, top: 0.3em),
      text(size: 0.8em, fill: red)[Error: #step.error_message],
    )
  ]

  if step.table != none {
    let tab = step.table
    box(inset: (left: 10pt), [
      #table(
        columns: table_columns(tab.columns),
        stroke: table_stroke,
        ..tab.rows.flatten()
      )
    ])
    [ \ ]
  }

  if step.doc_string != none [
    #render_description(step.doc_string) \
  ]
}

#let render_steps_table(steps) = {
  table(
    columns: (auto, 1.8fr, auto),
    stroke: table_stroke,
    inset: 8pt,
    align: (left, left, right),
    [Keyword], [Step], [Outcome],
    ..steps.map(step => (
      [#text(fill: blue, weight: "bold")[#step.keyword]],
      [
        #text(size: 0.95em)[#step.text]
        #render_step_details(step)
      ],
      [#outcome(step.outcome)],
    )).flatten(),
  )
}

#let render_background(background) = {
  if background.description != none [
    #render_description(background.description)
  ]

  if background.steps.len() > 0 [
    #render_steps_table(background.steps)
  ]
}

#let render_scenario(scenario, level: 3) = {
  if level == 4 {
    [==== #scenario.name]
  } else {
    [=== #scenario.name]
  }

  if scenario.description != none [
    #render_description(scenario.description)
  ]

  if scenario.tags.len() > 0 [
    #render_tags(scenario.tags)
  ]

  if scenario.steps.len() > 0 [
    #render_steps_table(scenario.steps)
  ] else [
    #emph[No steps defined for this scenario.]
  ]
}
