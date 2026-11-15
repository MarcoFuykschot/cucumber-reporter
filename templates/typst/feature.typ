#import sys: inputs

#import "outline.typ": *
#import "scenario.typ": *
#import "styles.typ": *

#show table.cell: set text(hyphenate: true)

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
        [== #feature.name]
      )
      #if index != none [
        #let feature-id = "feature-" + str(index)
        #label(feature-id)
      ]

      #render_tags(feature.tags)

      #render_description(feature.description)

      #let rules_table = feature.rules.enumerate().map(((rule-index, rule)) => {
        (
          [#(rule-index + 1)],
          [#rule.name],
          if rule.outcome != none [#outcome(rule.outcome)] else [],
        )
      })
      #let has-rule-scenarios = feature.has_rule_scenarios
      #if rules_table.len() > 0 [
        === Rules summary
        #table(
          columns: (auto, 1fr, auto),
          inset: 4pt,
          stroke: table_stroke,
          render_table_header([\#]),
          render_table_header([Rule]),
          render_table_header([Result]),
          ..rules_table.flatten(),
        )
      ]

      #let summary_rows = feature.summary_rows.map(row => {
        let result = if row.outcome != none [#outcome(row.outcome)] else []
        let errors = [#row.errors]
        if has-rule-scenarios {
          if row.rule == none {
            (table.cell(colspan: 2)[#row.scenario], result, errors)
          } else {
            ([#row.rule], [#row.scenario], result, errors)
          }
        } else {
          ([#row.scenario], result, errors)
        }
      })
      #if summary_rows.len() > 0 [
        === Scenario summary
        #let summary-columns = if has-rule-scenarios {
          (auto, 1fr, auto, 1.5fr)
        } else {
          (1fr, auto, 1.5fr)
        }
        #let summary-headers = if has-rule-scenarios {
          (
            render_table_header([Rule]),
            render_table_header([Scenario]),
            render_table_header([Result]),
            render_table_header([Errors]),
          )
        } else {
          (
            render_table_header([Scenario]),
            render_table_header([Result]),
            render_table_header([Errors]),
          )
        }
        #table(
          columns: summary-columns,
          inset: 4pt,
          stroke: table_stroke,
          ..summary-headers,
          ..summary_rows.flatten(),
        )
      ]

      #if feature.rules.len() > 0 [
        === Requirements
        #for rule in feature.rules [
          === #rule.name
          #if rule.description != none [
            #render_description(rule.description)
          ]
          #if rule.tags.len() > 0 [
            #render_tags(rule.tags)
          ]
          #for scenario in rule.scenarios [
            #render_element(scenario, level: 4)
          ]
        ]
      ]

      #if feature.background != none and (
        (feature.background.description != none and feature.background.description.len() > 0)
        or feature.background.steps.len() > 0
      ) [
        === Background
        #render_background(feature.background)
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
