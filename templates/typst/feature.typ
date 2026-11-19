#import sys: inputs

#import "outline.typ": *
#import "scenario.typ": *
#import "styles.typ": *

#show table.cell: set text(hyphenate: true)

#let render_feature(feature, index: none, headings: (:), table_headers: (:)) = {
  let render_element(element, level: 2) = {
    let target = if index != none {
      "feature-" + str(index) + "-scenario-" + str(element.line)
    } else {
      none
    }
    if element.keyword == "Scenario" {
      render_scenario(element, level: level, target: target, table_headers: table_headers)
    } else {
      if element.keyword == "Scenario Outline" {
        render_outline(element, level: level, target: target, table_headers: table_headers)
      }
    }
  }

  if feature == none [
    #emph[Feature data is missing in the inputs.]
  ] else {
    let target = if index != none { "feature-" + str(index) } else { none }
    [
      #heading(level: 1)[#feature.name]
      #if target != none { label(target) }

      #render_tags(feature.tags)

      #render_description(feature.description)

      #let has-rule-scenarios = feature.has_rule_scenarios
      #let rules_table = feature.rules.enumerate().map(((rule-index, rule)) => {
        let rule-key = str(rule-index + 1)
        let rule_result = if rule.outcome != none [#outcome(rule.outcome)] else []
        let rule_row = (table.cell(colspan: 2)[#rule.name], rule_result)
        let rule_scenarios = feature.summary_rows.filter(summary => summary.rule == rule-key)
        let scenario_rows = rule_scenarios.map(summary => {
            let result = if summary.outcome != none [#outcome(summary.outcome)] else []
            let scenario = {
              if summary.line != none and index != none {
                let target = "feature-" + str(index) + "-scenario-" + str(summary.line)
                link(label(target))[#render_outline_text(summary.scenario)]
              } else {
                render_outline_text(summary.scenario)
              }
            }
            ([], scenario, result)
          })
        (rule_row, ..scenario_rows)
      }).flatten().flatten()
      #if rules_table.len() > 0 [
        #if has-rule-scenarios [
          == #headings.at("rules_and_scenarios_summary", default: "Rules and scenarios summary")
        ] else [
          == #headings.at("rules_summary", default: "Rules summary")
        ]
        #table(
          columns: (auto, 1fr, auto),
          inset: 4pt,
          stroke: table_stroke,
          render_table_header(table_headers.at("rule", default: "Rule")),
          render_table_header(table_headers.at("scenario", default: "Scenario")),
          render_table_header(table_headers.at("result", default: "Result")),
          ..rules_table,
        )
      ]

      #let feature_scenarios = feature.summary_rows.filter(summary => summary.rule == none)
      #let summary_rows = feature_scenarios.map(summary => {
        let result = if summary.outcome != none [#outcome(summary.outcome)] else []
        let scenario = {
          if summary.line != none and index != none {
            let target = "feature-" + str(index) + "-scenario-" + str(summary.line)
            link(label(target))[#render_outline_text(summary.scenario)]
          } else {
            render_outline_text(summary.scenario)
          }
        }
        (scenario, result)
      })
      #if summary_rows.len() > 0 [
        == #headings.at("scenario_summary", default: "Scenario summary")
        #table(
          columns: (1fr, auto),
          inset: 4pt,
          stroke: table_stroke,
          render_table_header(table_headers.at("scenario", default: "Scenario")),
          render_table_header(table_headers.at("result", default: "Result")),
          ..summary_rows.flatten(),
        )
      ]

      #if feature.rules.len() > 0 [
        #for rule in feature.rules [
          == #rule.name
          #if rule.description != none [
            #render_description(rule.description)
          ]
          #if rule.tags.len() > 0 [
            #render_tags(rule.tags)
          ]
          #for scenario in rule.scenarios [
            #render_element(scenario, level: 3)
          ]
        ]
      ]

      #if feature.background != none and (
        (feature.background.description != none and feature.background.description.len() > 0)
        or feature.background.steps.len() > 0
      ) [
        == #headings.at("background", default: "Background")
        #render_background(feature.background, table_headers: table_headers)
      ]

      #if feature.scenarios.len() > 0 [
        == Scenarios
        #for scenario in feature.scenarios [
          #render_element(scenario)
        ]
      ]
    ]
  }
}
