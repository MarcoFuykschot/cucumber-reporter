#import "@preview/cmarker:0.1.10"
#import "styles.typ": *

#let render_step(step) = [
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
  #if step.table != none {
    let tab = step.table
    box(
    inset: (left: 10pt),
    [#table(
      columns: tab.columns,
      ..tab.rows.map(row => row.map(cell => cell)).flatten()
    )]
    )
  }
]

#let render_scenario(scenario) = [
  === Scenario: #scenario.name
  #render_tags(scenario.tags)
  #if scenario.description != none [
    #cmarker.render(scenario.description)
  ]

  #if scenario.steps.len() > 0 [
    #for step in scenario.steps {
      render_step(step)
    }
  ] else [
    #emph[No steps defined for this scenario.]
  ]
]
