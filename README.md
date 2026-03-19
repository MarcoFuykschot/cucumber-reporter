## Goal

To create a reporter for the crate cucumber, that directly
produces html files.

for each execute feature an html file is produced using the
name of the feature. At the end an index file is produced
with all executed features and some stats.

You can use the commandline option --output-html-path to change the output
path of the html files, default it uses the current directory where the tests
are run.

## Parameters usage
```
Usage: cargo test -- <Options>

Options:
    --output-html-path <PATH>       The output path of the generated HTML files.
    --template <PATH>               The location of the files for templating.
```

## Examples

### A simple gherkin feature file

<!--CONTENT-START:features/feature-simple.feature:Feature-->
```Feature
Feature: simple
    With a description

    Scenario: Scenario 1
        Given a fact
        When something is executed
        Then the result is oke

    Scenario: Scenario 2
        Given a fact
        And a other fact
        When something is executed
        Then the result is failed
        
     Scenario: Scenario 3
        Given a fact
        Then a Skipped line
```
<!--CONTENT-END:features/feature-simple.feature-->

Will produce the following [html output](https://marcofuykschot.github.io/cucumber-reporter/F216684217177122904.html)

### A gherkin feature using an outline

<!--CONTENT-START:features/feature-with-outline.feature:Feature-->
```Feature
Feature: outline

    Scenario Outline: Scenario Outline name <test>

        Given a fact with '<Header1>'

        Given a fact with '<Header2>'

        Examples:
            | Header1 | Header2 | test |
            | Value 1 | Value 1 | 1    |
            | Value 2 | Value 2 | 2    |
            | Value 3 | Value 3 | 3    |
            | Value 4 | Value 4 | 4    |

    Scenario Outline: Scenario with skipped
        Given a skipped fact with '<Header1>'

        Examples:
            | Header1 | Header2 | test |
            | Value 3 | Value 3 | 3    |
            | Value 4 | Value 4 | 4    |


    Scenario Outline: Scenario with skipped extra new line
        Given a skipped fact with '<Header1>'

        Examples:

            | Header1 | Header2 | test |

            | Value 3 | Value 3 | 3 |

            | Value 5 | Value 4 | 4 |

```
<!--CONTENT-END:features/feature-with-outline.feature-->

Will produce the following [html output](https://marcofuykschot.github.io/cucumber-reporter/F13495275682151091117.html)

## Templating
Currenlty templating functions by using html with basic css.
For templating, the [handlebars](https://docs.rs/handlebars/latest/handlebars/) rust crate is used.
For an example to use it. Refer to the default template in the `templates/` folder.

## planned
* direct pdf output
* markdown in descriptions
* doc stringsc stringsrings

## References

#### Cucumber
* [Cucumber-rs book](https://cucumber-rs.github.io/cucumber/current/)
#### Templating  
* [Handlebars](https://docs.rs/handlebars/latest/handlebars/) - crate used.
* [Handlebarsjs](https://handlebarsjs.com/guide/) - better guide for templating.

