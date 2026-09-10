#import sys: inputs

#import "outline.typ": *
#import "scenario.typ": *

#let feature(feature) = {

  let render(element) = {
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
      = Feature: #feature.name

      #feature.description

      Outcome: #feature.outcome

      #if not feature.background == none [
        == Background
        #let background = feature.background
        #background.description
        #if background.steps.len() > 0 [
          #step(background.steps)
        ] else [
          #emph[No steps defined for the background.]
        ]
      ]

      #if feature.scenarios.len() > 0 [
        == Scenarios
        #for scenario in feature.scenarios [
          #render(scenario)
        ]
      ]
    ]
  }
}
