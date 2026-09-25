
#let outcome(value) = {
  show "passed": outcome => text(fill: green, weight: "bold")[#outcome]
  show "failed": outcome => text(fill: red, weight: "bold")[#outcome]
  show "skipped": outcome => text(fill: gray, weight: "bold")[#outcome]
  [#value]
}

#let render_tags(tags) = {
  if tags.len() > 0 [
    Tags: #tags.join(" ") \
  ]
}