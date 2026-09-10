
#import "styles.typ": *

#let step(steps) = {
  for step in steps [
    #{
      show regex("Given|When|Then"): keyword => box(
        width: 45pt,
        text(fill: blue, weight: "bold")[#keyword:],
      )
      show regex("And|But"): keyword => box(
        inset: (left: 5pt),
        width: 45pt,
        text(fill: blue, weight: "bold")[#keyword:],
      )
      [#step.keyword]
    }
    #step.text
    #outcome([#step.outcome]) \
  ]
}

#let render_scenario(scenario) = [
  === Scenario: #scenario.name

  #scenario.description

  #if scenario.steps.len() > 0 [
    #step(scenario.steps)
  ] else [
    #emph[No steps defined for this scenario.]
  ]
]
