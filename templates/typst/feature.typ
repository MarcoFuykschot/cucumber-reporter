#import sys: inputs

#import "outline.typ": *
#import "scenario.typ": *
#import "styles.typ": *

#let render_feature(feature, index: none) = {
  let render_element(element, level: 3) = {
    if element.keyword == "Scenario" {
      render_scenario(element, level: level)
    } else {
      if element.keyword == "Scenario Outline" {
        render_outline(element, level: level)
      }
    }
  }

  if feature == none [
    #emph[Feature data is missing in the inputs.]
  ] else {
    [
      #grid(
        columns: (auto, auto),
        gutter: 0.5em,
        align: (left, bottom),
        [== #feature.name],
        if feature.outcome != none [#outcome(feature.outcome)] else [],
      )
      #if index != none [
        #let feature-id = "feature-" + str(index)
        #label(feature-id)
      ]

      #render_tags(feature.tags)

      #render_description(feature.description)

      #let rules_table = feature.rules.map(rule => {
        let step_outcomes = rule.scenarios.map(scenario => {
          let outcomes = scenario.steps.map(step => step.outcome)
          if outcomes.any(value => value == "failed") { "failed" }
          else if outcomes.any(value => value == "skipped") { "skipped" }
          else { "passed" }
        })
        let outcome_value = if step_outcomes.any(value => value == "failed") { "failed" }
          else if step_outcomes.any(value => value == "skipped") { "skipped" }
          else { "passed" }
        ([#rule.name], [#outcome(outcome_value)])
      })
      #if rules_table.len() > 0 [
        === Rules summary
        #table(
          columns: (1fr, auto),
          inset: 6pt,
          stroke: table_stroke,
          [Rule], [Outcome],
          ..rules_table.flatten(),
        )
      ]

      #let summary_rows = (
        feature.scenarios.map(scenario => {
          let outcome_value = if scenario.steps.len() > 0 {
            let outcomes = scenario.steps.map(step => step.outcome)
            if outcomes.any(value => value == "failed") { "failed" }
            else if outcomes.any(value => value == "skipped") { "skipped" }
            else { "passed" }
          } else {
            "passed"
          }
          let errors = feature.errors.filter(error => error.scenario == scenario.name).map(error => error.message).join("\n")
          ([#scenario.name], [#outcome(outcome_value)], [#errors])
        })
        + feature.rules.map(rule => rule.scenarios.map(scenario => {
          let outcome_value = if scenario.steps.len() > 0 {
            let outcomes = scenario.steps.map(step => step.outcome)
            if outcomes.any(value => value == "failed") { "failed" }
            else if outcomes.any(value => value == "skipped") { "skipped" }
            else { "passed" }
          } else {
            "passed"
          }
          let errors = feature.errors.filter(error => error.scenario == scenario.name).map(error => error.message).join("\n")
          ([#rule.name - #scenario.name], [#outcome(outcome_value)], [#errors])
        })).flatten()
        + feature.errors.map(error => ([#error.scenario], [#outcome("failed")], [#error.message]))
      )
      #if summary_rows.len() > 0 [
        === Scenario summary
        #table(
          columns: (1fr, auto, 1.5fr),
          inset: 6pt,
          stroke: table_stroke,
          [Scenario], [Outcome], [Errors],
          ..summary_rows.flatten(),
        )
      ]

      #for rule in feature.rules [
        === Rule: #rule.name
        #render_tags(rule.tags)
        #render_description(rule.description)
        #for scenario in rule.scenarios [
          #render_element(scenario, level: 4)
        ]
      ]

      #if not feature.background == none [
        === Background
        #let background = feature.background
        #background.description
        #if background.steps.len() > 0 [
          #for step in background.steps [
            #render_step(step)
          ]
        ] else [
          #emph[No steps defined for the background.]
        ]
      ]

      #if feature.scenarios.len() > 0 [
        === Scenarios
        #for scenario in feature.scenarios [
          #render_element(scenario, level: 3)
        ]
      ]
    ]
  }
}
