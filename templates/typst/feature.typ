#import sys: inputs

#import "outline.typ": *
#import "scenario.typ": *
#import "styles.typ": *

#let render_feature(feature, index: none) = {
  let render_element(element) = {
    if element.keyword == "Scenario" {
      render_scenario(element)
    } else {
      if element.keyword == "Scenario Outline" {
        render_outline(element)
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

      #if feature.errors != none and feature.errors.len() > 0 [
        === Errors
        #table(
          columns: (auto, 1fr),
          inset: 6pt,
          fill: (x, y) => if y == 0 { rgb("#fbe8e8") } else { white },
          stroke: table_stroke,
          [#text(fill: red, weight: "bold")[Scenario]],
          [#text(fill: red, weight: "bold")[Message]],
          ..feature.errors.map(error => (
            [#text(fill: red)[#error.scenario]],
            [#text(fill: red)[#error.message]],
          )).flatten(),
        )
      ]

      #render_description(feature.description)

      #for rule in feature.rules {
        [== Rule: #rule.name
          #render_tags(rule.tags)
          #render_description(rule.description)
          #for scenario in rule.scenarios [
            #render_element(scenario)
          ]
        ]
      }

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
          #render_element(scenario)
        ]
      ]
    ]
  }
}
