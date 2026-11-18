#import "styles.typ": *

#show table.cell: set text(hyphenate: true)

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
    table(
      columns: table_columns(tab.columns),
      stroke: table_stroke,
      inset: 2pt,
      ..tab.rows.flatten()
    )
  }
  if step.doc_string != none [
    #render_description(step.doc_string)
    #linebreak()
  ]
}

#let render_steps_table(steps, table_headers, show_outcome: true) = {
  let columns = if show_outcome { (auto, 1.8fr, auto) } else { (auto, 1.8fr) }
  let headers = if show_outcome {
    (
      table.cell(
        colspan: 2,
        render_table_header(table_headers.at("step", default: "Step")),
      ),
      render_table_header(table_headers.at("result", default: "Result")),
    )
  } else {
    (
      table.cell(
        colspan: 2,
        render_table_header(table_headers.at("step", default: "Step")),
      ),
    )
  }

  table(
    columns: columns,
    stroke: table_stroke,
    inset: 4pt,
    align: if show_outcome { (left, left, right) } else { (left, left) },
    ..headers,
    ..steps
      .map(step => {
        let keyword = [#text(fill: blue, weight: "bold")[#step.keyword]]
        let description = [
          #text(size: 0.95em)[#step.text]
          #render_step_details(step)
        ]
        if show_outcome {
          (keyword, description, [#outcome(step.outcome)])
        } else {
          (keyword, description)
        }
      })
      .flatten(),
  )
}

#let render_background(background, table_headers: (:)) = {
  if background.description != none [
    #render_description(background.description)
  ]

  if background.steps.len() > 0 [
    #render_steps_table(background.steps, table_headers)
  ]
}

#let render_scenario(
  scenario,
  level: 2,
  show_step_outcome: true,
  target: none,
  table_headers: (:),
) = {
  [
    #heading(level: level)[#scenario.name]
    #if target != none { label(target) }
  ]

  if scenario.description != none [
    #render_description(scenario.description)
  ]

  if scenario.tags.len() > 0 [
    #render_tags(scenario.tags)
  ]

  if scenario.steps.len() > 0 [
    #render_steps_table(
      scenario.steps,
      table_headers,
      show_outcome: show_step_outcome,
    )
  ] else [
    #emph[No steps defined for this scenario.]
  ]
}
