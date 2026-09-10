
#let outcome(value) = {
  show "passed": outcome => text(fill: green, weight: "bold")[#outcome]
  show "failed": outcome => text(fill: red, weight: "bold")[#outcome]
  show "skipped": outcome => text(fill: gray, weight: "bold")[#outcome]
  [#value]
}